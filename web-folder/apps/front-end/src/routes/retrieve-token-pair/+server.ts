import { json, type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = ({ cookies }) => {
	const refresh = cookies.get('refresh');

	cookies.set('refresh', 'meow', { path: '/' });

	return json({
		token:
			'eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6IjAxOTk0M2RiLTJmMzctNzNlMC1hN2Y4LTkwYmU0MjVmZjQ5MyJ9.eyJzdWIiOiIwMTk5NDNkYi0yZjM4LTc5MTAtYTE2Yy04ZGE3MGUzMTM2YWIiLCJpZGVudCI6IkpvaG4gQnVyZ2VyIFRoZSBGb3VydGgiLCJhdmF0YXIiOiJodHRwczovL2V4YW1wbGUuY29tIiwiZXhwIjoxNzU3NzgwOTYyLCJpYXQiOjE3NTc3ODAwNjIsImVtYWlsIjoibWVAbWlsYW5lZ2Jlcmluay5jb20ifQ.mGmOY0ebzE0ZrZNWF1gNwczbxkoZDTiBbLF0NwJ1e8q6klFln_k5wPFBLav0Q3qjfyH7U2Yyq-1awK_7JK2CAg'
	});
};
