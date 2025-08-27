import { json } from '@sveltejs/kit';
import type { RequestHandler } from '../retrieve-token-pair/$types';

export const GET: RequestHandler = ({ request }) => {
	const x = request.headers.get('Authorization');
	console.log(x);

	return json({ x });
};
