/* Service worker specific import that doesn't call window anywhere */
/// <reference lib="webworker" />
import { decodeJwt } from 'jose';
import { type SWRequest, SWReqType } from './request.js';
import { Err, Ok, type Result, type PromiseResult } from '$lib/result.js';
import { user, type UserForLogin } from '$lib/schemas/index.js';
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
	Fetch = 'fetch'
}

export function mountServiceWorker(self: ServiceWorkerGlobalScope) {
	self.addEventListener(SWEvent.Install, () => {
		self.skipWaiting();
	});

	self.addEventListener(SWEvent.Message, async (event: SWMessageEvent<SWRequest>) => {
		const { ports, data } = event;
		const port = ports[0];

		if (!port) return;
		switch (data.type) {
			case SWReqType.SetToken: {
				const result = await refreshTokens();
				port.postMessage(result);
				break;
			}
			case SWReqType.GetUser: {
				if (!token) {
					port.postMessage(Err(new Error('Missing token')));
					return;
				}

				const claims = decodeJwt(token);
				const parsedUser = user.parse(claims);
				port.postMessage(Ok(parsedUser));
				break;
			}
			case SWReqType.LoginRequest: {
				const user = data.payload;
				const result = await loginUser(user);

				port.postMessage(result);
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

	function x(targetUnix: number) {
		targetUnix *= 1000;

		const now = Date.now();
		const delay = targetUnix - now;

		setTimeout(refreshTokens, delay);
	}
}

async function loginUser(user: UserForLogin): PromiseResult<void> {
	return Ok(undefined);
}
