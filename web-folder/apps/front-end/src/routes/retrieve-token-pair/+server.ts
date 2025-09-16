import { json, type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ cookies }) => {
	await new Promise((resolve) => setTimeout(resolve, 5000));

	const refresh = cookies.get('refresh');

	cookies.set('refresh', 'meow', { path: '/' });

	return json({
		token:
			'eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6IjAxOTk1MDA0LTk0OWUtNzRhMC05YWYyLTM4ZGIyMTJhOTU2ZCJ9.eyJzdWIiOiIwMTk5NTAwNC05NDllLTc0YTAtOWFmMi0zOGUyMWFmMDU4NDUiLCJpZGVudCI6Ik1lb3cgbWVvdyIsImF2YXRhciI6Imh0dHBzOi8vZXhhbXBsZS5jb20iLCJleHAiOjE3NTc5ODUwMDEsImlhdCI6MTc1Nzk4NDEwMSwiZW1haWwiOiJtZUBtaWxhbmVnYmVyaW5rLmNvbSJ9.dY0kpiftuXvM1slQ4sNKEF87f9ap87P5uH1bufDs4JOHoJ_N5qQDOnzG_B9SWfzov9ncCP90nFIIaOyd__a9Dw'
	});
};
