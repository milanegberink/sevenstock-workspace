import { post } from '@lib/core';

export const POST = async (event) => {
	let sessionId = event.cookies.get('session');

	const res = await post('/login');
};
