import { json, type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = ({ cookies }) => {
	const refresh = cookies.get('refresh');

	return json({ token: refresh });
};
