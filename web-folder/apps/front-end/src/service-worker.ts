/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />
//
import { decodeJwt } from 'jose';
import { type TokenClaims, user } from '@lib/core/schemas';
import { type Result, Err, Ok } from '@lib/core/service-worker';
import { SwRequest } from '../src/lib/service-worker/request';

const sw = self as unknown as ServiceWorkerGlobalScope;

let token: string | null = null;

sw.addEventListener('install', () => {
	sw.skipWaiting();
});

sw.addEventListener('message', async (event: ExtendableMessageEvent) => {
	const { ports, data } = event;
	const port = ports[0];

	if (!port) return;
	switch (data.type) {
		case SwRequest.SetToken: {
			const result = await refreshTokens();
			port.postMessage(result);
			break;
		}
		case SwRequest.GetUser: {
			if (!token) {
				port.postMessage(Err(new Error('Missing token')));
				return;
			}

			const claims: TokenClaims = decodeJwt(token);
			const parsedUser = user.parse(claims);
			port.postMessage(Ok(parsedUser));
			break;
		}
	}
});

sw.addEventListener('fetch', (event: FetchEvent) => {
	if (!token) return;

	const headers = new Headers(event.request.headers);
	headers.append('Authorization', `Bearer ${token}`);

	const request = new Request(event.request, {
		headers
	});

	event.respondWith(fetch(request));
});

async function refreshTokens(): Promise<Result<void, Error>> {
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
