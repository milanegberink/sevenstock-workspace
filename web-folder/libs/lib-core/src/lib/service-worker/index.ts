/* Service worker specific import that doesn't call window anywhere */
/// <reference lib="webworker" />
import { decodeJwt } from 'jose';
import { type SWRequest, SWReqType } from './request.js';
import { Err, Ok, type Result, type PromiseResult } from '$lib/result.js';
import { user, type LoginPayload } from '$lib/schemas/index.js';
import { post } from '$lib/better-fetch.svelte';
export * from '../result.js';
export * from './request.js';
export { sendRequest } from './make-request.js';

let token: string | null = null;

interface SWMessageEvent<T> extends ExtendableMessageEvent {
	data: T;
}

enum SWEvent {
	Install = 'install',
	Message = 'message',
	Fetch = 'fetch',
	Activate = 'activate'
}

export function mountServiceWorker(self: ServiceWorkerGlobalScope) {
	self.addEventListener(SWEvent.Install, () => {
		self.skipWaiting();
	});

	self.addEventListener(SWEvent.Activate, (event) => {
		event.waitUntil(self.clients.claim());
	});

	self.addEventListener(SWEvent.Message, async (event: SWMessageEvent<SWRequest>) => {
		const { ports, data } = event;
		const port = ports[0];

		if (!port) return;
		switch (data.type) {
			case SWReqType.SetToken: {
				if (token) {
					const claims = decodeJwt(token);
					const parsedUser = await user.safeParseAsync(claims);

					if (parsedUser.error) return Err(new Error(parsedUser.error.message));

					port.postMessage(Ok(parsedUser.data));
					return;
				}
				const result = await refreshTokens();

				if (!result.ok) {
					port.postMessage(result);
					return;
				}

				if (!token) {
					port.postMessage(Err(new Error('Missing token')));
					return;
				}

				const claims = decodeJwt(token);
				const parsedUser = await user.safeParseAsync(claims);

				if (parsedUser.error) return Err(new Error(parsedUser.error.message));

				port.postMessage(Ok(parsedUser.data));

				break;
			}
			case SWReqType.LoginRequest: {
				const res = await loginUser(data.payload);

				if (!res.ok) {
					port.postMessage(Err('Failed to login'));
					return;
				}

				token = res.value.access_token;

				const claims = decodeJwt(token);
				const parsedUser = await user.safeParseAsync(claims);

				if (parsedUser.error) return Err(new Error(parsedUser.error.message));

				port.postMessage(Ok(parsedUser.data));
			}
		}
	});

	self.addEventListener(SWEvent.Fetch, (event: FetchEvent) => {
		if (!token) return;

		const headers = new Headers(event.request.headers);
		headers.append('Authorization', `Bearer ${token}`);

		const request = new Request(event.request, {
			headers
		});

		event.respondWith(fetch(request));
	});

	async function refreshTokens(): PromiseResult<void> {
		const res = await fetch('/retrieve-token-pair');
		const data = await res.json();
		token = data.token;
		return Ok(undefined);
	}

	function runAtTimestamp(targetUnix: number) {
		targetUnix *= 1000;

		const now = Date.now();
		const delay = targetUnix - now;

		setTimeout(refreshTokens, delay);
	}
}

async function loginUser(payload: LoginPayload): PromiseResult<{ access_token: string }> {
	const url = new URL('http://localhost:3000/auth/login');
	const res = await post(url, payload);
	if (!res.ok) return Err(new Error('Login fail'));

	return res;
}
