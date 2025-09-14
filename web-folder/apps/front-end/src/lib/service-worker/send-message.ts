import { type TokenClaims, type User } from '@lib/core/schemas';
import { type PromiseResult, type Result, Err } from '@lib/core';
import { SwRequest } from './request';
export { SwRequest };

export async function sendMessageToSw<T>(req: SwRequest): PromiseResult<T> {
	const controller = navigator.serviceWorker?.controller;
	if (!controller) return Err(new Error('No service worker active'));

	const result = new Promise<Result<T>>((resolve) => {
		const channel = new MessageChannel();

		channel.port1.onmessage = (event: MessageEvent<Result<T>>) => {
			const result = event.data;

			resolve(result);
		};
		controller.postMessage({ type: req }, [channel.port2]);
	});

	return result;
}
