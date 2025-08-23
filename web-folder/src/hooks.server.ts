import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	console.log(event.request.headers.get('Authorization'));
	const res = await resolve(event);
	return res;
};
