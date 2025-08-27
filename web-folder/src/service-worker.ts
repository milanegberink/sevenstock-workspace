import { decodeJwt, jwtVerify } from 'jose';

let token = null;

self.addEventListener('install', () => {
	self.skipWaiting();
});

self.addEventListener('message', (event) => {
	const msg = event.data;
	switch (msg) {
		case 'SET_TOKEN':
			refreshTokens();
			break;

		case 'GET_USER': {
			if (!token) return;
			const payload = decodeJwt(token);
			self.postMessage(payload);
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
