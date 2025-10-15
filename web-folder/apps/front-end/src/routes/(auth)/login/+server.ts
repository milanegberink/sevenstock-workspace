import { post } from '@lib/core';

export const POST = async ({ request }) => {
	const data = await request.json();

	const res = await post('/login');
};
