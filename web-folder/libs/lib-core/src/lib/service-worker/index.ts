/* Service worker specific import that doesn't call window anywhere */
/// <reference lib="webworker" />
import { decodeJwt } from 'jose';
import { type SWRequest, SWReqType } from './request.js';
import { Err, Ok, type Result, type PromiseResult } from '$lib/result.js';
import { user } from '$lib/schemas/index.js';
import type { LoginPayload, LoginResponse, User } from '$lib/schemas/index.js';
import { post } from '$lib/better-fetch.svelte';
export * from '../result.js';
export * from './request.js';
export { sendRequest } from './make-request.js';

class Config {
	token: string;
	user: User;
	constructor(token: string, user: User) {
		this.token = token;
		this.user = user;
	}
}
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
	let token: string | undefined;
	let authResult: PromiseResult<{ access_token: string }>;

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
					authResult = Promise.resolve(Ok({ access_token: token }));
				} else {
					authResult = refreshTokens();
				}
				break;
			}
			case SWReqType.LoginRequest: {
				authResult = loginUser(data.payload);
				break;
			}
		}

		const result = await authResult;

		if (!result.ok) {
			port.postMessage(result);
			return;
		}

		const newToken = result.value.access_token;

		token = newToken;

		const userResult = userFromToken(token);

		port.postMessage(userResult);
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

	async function refreshTokens(): PromiseResult<LoginResponse> {
		const url = new URL('http://localhost:3000/auth/exchange-refresh');

		const res = await post<undefined, LoginResponse>(url);

		return res;
	}

	function runAtTimestamp(targetUnix: number, fn: () => any) {
		const targetTime = targetUnix * 1000;
		const now = Date.now();

		const delay = Math.max(targetTime - now, 0);
		setTimeout(fn, delay);
	}
}

async function loginUser(payload: LoginPayload): PromiseResult<LoginResponse> {
	const url = new URL('http://localhost:3000/auth/login');
	const res = await post<LoginPayload, LoginResponse>(url, payload);
	if (!res.ok) return Err(new Error('Login fail'));

	return res;
}

function userFromToken(token: string): Result<User> {
	const claims = decodeJwt(token);

	const parsedUser = user.safeParse(claims);

	if (parsedUser.error) return Err(new Error(parsedUser.error.message));

	return Ok(parsedUser.data);
}
