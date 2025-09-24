import type { Result, PromiseResult } from './result.js';
import { ok, err } from './result.js';

class HttpError extends Error {
	constructor() {
		super();
	}
}

enum Method {
	Get = 'GET',
	Post = 'POST'
}

async function request<T, R>(endpoint: URL, method: Method, body?: T): PromiseResult<R> {
	let headers = new Headers({ 'Content-Type': 'application/json' });
	const opts: RequestInit = {
		method,
		headers,
		credentials: 'include',
		body: JSON.stringify(body)
	};

	const res = await fetch(endpoint, opts);

	if (!res.ok) {
		console.log(res);
		return err(new HttpError());
	}

	const data: R = await res.json();
	return ok(data);
}

export const get = async <R>(endpoint: URL): PromiseResult<R> => {
	return request(endpoint, Method.Get);
};

export const post = async <T, R>(endpoint: URL, body?: T): PromiseResult<R> => {
	return request(endpoint, Method.Post, body);
};
