/// <reference lib="webworker" />

import { decodeJwt } from 'jose';
import { type TokenClaims, user } from 'shared-schemas';

let token: string | null = null;

self.addEventListener('install', function (this: ServiceWorkerGlobalScope) {
	this.skipWaiting();
});

self.addEventListener('message', (event) => {
	const { ports, data } = event;
	switch (data) {
		case 'SET_TOKEN':
			refreshTokens();
			break;

		case 'GET_USER': {
			if (!token) return;
			const claims: TokenClaims = decodeJwt(token);
			const parsedUser = user.parse(claims);
			ports[0]?.postMessage(parsedUser);
			break;
		}
	}
});

self.addEventListener('fetch', (event) => {
	if (!token) return;

	const headers = new Headers(event.request.headers);
	headers.append('Authorization', `Bearer ${token}`);

	const request = new Request(event.request, {
		headers
	});

	event.respondWith(fetch(request));
});

async function refreshTokens() {
	const res = await fetch('/retrieve-token-pair');
	const data = await res.json();
	token = data.token;
}

function x(targetUnix: number) {
	targetUnix *= 1000;

	const now = Date.now();
	const delay = targetUnix - now;

	setTimeout(refreshTokens, delay);
}
