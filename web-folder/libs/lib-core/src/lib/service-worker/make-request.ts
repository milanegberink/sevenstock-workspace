import { type SWRequest } from './request.js';
import { type PromiseResult, type Result, err } from '$lib/result.js';

export async function sendRequest<T>(req: SWRequest): PromiseResult<T> {
	await navigator.serviceWorker.ready;

	const controller = navigator.serviceWorker.controller;

	if (!controller) {
		return err(new Error('No service worker active'));
	}

	const result = new Promise<Result<T>>((resolve) => {
		const channel = new MessageChannel();

		channel.port1.onmessage = (event: MessageEvent<Result<T>>) => {
			const result = event.data;

			resolve(result);
		};
		controller.postMessage(req, [channel.port2]);
	});

	return result;
}
