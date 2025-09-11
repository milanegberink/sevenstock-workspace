import { type TokenClaims, type User } from 'shared-schemas';

export async function setToken() {
	if ('serviceWorker' in navigator) {
		const registration = await navigator.serviceWorker.ready;
		registration.active?.postMessage('SET_TOKEN');
	}
}
