import type { Result, PromiseResult } from './result.js';
import { ok, err } from './result.js';
import { PUBLIC_BACKEND_URL } from '$env/static/public';

const defaultHeaders = {
	'Content-Type': 'application/json'
};

class HttpError extends Error {
	constructor() {
		super();
	}
}

enum Method {
	Get = 'GET',
	Post = 'POST'
}

function buildURL(url: string | URL) {
	if (url instanceof URL) {
		return url;
	}

	if (url.startsWith('https://')) {
		return new URL(url);
	}

	return new URL(url, PUBLIC_BACKEND_URL);
}

async function request<T, R>(
	endpoint: string | URL,
	method: Method,
	body?: T,
	headers?: Headers
): PromiseResult<R> {
	const options: RequestInit = {
		method,
		headers: headers ? headers : defaultHeaders,
		body: JSON.stringify(body)
	};

	endpoint = buildURL(endpoint);

	const res = await fetch(endpoint, options);

	if (!res.ok) {
		console.log(res);
		return err(new HttpError());
	}

	const data: R = await res.json();
	return ok(data);
}

export const get = async <R>(endpoint: string | URL): PromiseResult<R> => {
	return request(endpoint, Method.Get);
};

export const post = async <T, R>(
	endpoint: string | URL,
	body?: T,
	opts?: RequestInfo
): PromiseResult<R> => {
	return request(endpoint, Method.Post, body);
};
