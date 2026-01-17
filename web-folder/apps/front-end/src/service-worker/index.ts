import { decodeJwt } from 'jose';
import { type SWRequest, SWReqType } from './request.ts';
import { err, ok, type Result, type PromiseResult } from './result.js';
// import { user } from '$lib/schemas/index.js';
// import type { LoginPayload, LoginResponse, User } from '$lib/schemas/index.js';
// import { post } from '$lib/better-fetch.svelte';
export * from './result.ts';
export * from './request.ts';
export { sendRequest } from './make-request.ts';

class Config {
	token?: string;
	user?: User;
	constructor({ token, user }: { token?: string; user?: User }) {
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

interface AuthResponse {
	access_token: string;
}

export function mountServiceWorker(self: ServiceWorkerGlobalScope) {
	const config = new Config({ token: undefined, user: undefined });

	let authResult: PromiseResult<AuthResponse>;

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
				if (config.token) {
					authResult = Promise.resolve(ok({ access_token: config.token }));
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

		config.token = newToken;

		const userResult = userFromToken(config.token);

		if (!userResult.ok) {
			port.postMessage(userResult);
			return;
		}

		config.user = userResult.value;

		port.postMessage(userResult);
	});

	self.addEventListener(SWEvent.Fetch, (event: FetchEvent) => {
		if (!config.token || event.request.url !== PUBLIC_BACKEND_URL) return;

		const headers = new Headers(event.request.headers);
		headers.append('Authorization', `Bearer ${config.token}`);

		const request = new Request(event.request, {
			headers
		});

		event.respondWith(fetch(request));
	});

	async function refreshTokens(): PromiseResult<LoginResponse> {
		const url = new URL(`${PUBLIC_BACKEND_URL}/auth/exchange-refresh`);

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
	const url = new URL(`${PUBLIC_BACKEND_URL}/auth/login`);
	const res = await post<LoginPayload, LoginResponse>(url, payload);
	if (!res.ok) return err(new Error('Login fail'));

	return res;
}

function userFromToken(token: string): Result<User> {
	const claims = decodeJwt(token);

	const parsedUser = user.safeParse(claims);

	if (parsedUser.error) return err(new Error(parsedUser.error.message));

	return ok(parsedUser.data);
}
