import { json, type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = ({ cookies }) => {
	const refresh = cookies.get('refresh');

	return json({
		token:
			'eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6IjAxOTkzYjBhLTcxMWMtNzA1Mi05NmE2LWI3ZTY4YTQ2NjA5NyJ9.eyJzdWIiOiIwMTk5M2IwYS03MTFjLTcwNTItOTZhNi1iN2Y5NWMyNTFjNzYiLCJpZGVudCI6Ik1pbGFuIiwiYXZhdGFyIjoiaHR0cHM6Ly9leGFtcGxlLmNvbSIsImV4cCI6MTc1NzYzMzA2NCwiaWF0IjoxNzU3NjMyMTY0LCJlbWFpbCI6Im1lQG1pbGFuZWdiZXJpbmsuY29tIn0.Ee_FV8Hq_vbfDa0KDR6RvnNkgjzL1U41bMa-mU-CGvSZaFHYJR8vPxloolMg7Tiq8txWmfR0IafWzBqFX__EAw'
	});
};
