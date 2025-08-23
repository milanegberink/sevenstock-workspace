/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
/// <reference lib="webworker" />
//
/// <reference types="@sveltejs/kit" />

declare let self: ServiceWorkerGlobalScope;

const token = 'secret-toke2n';

self.addEventListener('install', () => {
	self.skipWaiting();
});

self.addEventListener('fetch', (event) => {
	const headers = new Headers(event.request.headers);
	headers.append('Authorization', `Bearer ${token}`);

	const request = new Request(event.request, {
		headers
	});

	event.respondWith(fetch(request));
});
