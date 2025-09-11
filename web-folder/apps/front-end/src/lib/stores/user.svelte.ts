import { type User } from 'shared-schemas';

let user: User | null = $state(null);

export async function getUser(): Promise<User> {
	if (!user) {
		if ('serviceWorker' in navigator) {
			const channel = new MessageChannel();

			const registration = await navigator.serviceWorker.ready;

			registration.active?.postMessage('GET_USER', [channel.port2]);

			user = await new Promise<User>((resolve) => {
				channel.port1.onmessage = (event) => {
					resolve(event.data);
				};
			});
		}
		if (!user) {
			throw new Error();
		}
	}

	return user;
}
