import { goto } from '$app/navigation';
import { sendRequest, getUserRequest } from '@lib/core/service-worker';
import { Err, Ok, type PromiseResult } from '@lib/core';
import { type User } from '@lib/core/schemas';

let user: User | null = $state(null);

export async function getUser(): PromiseResult<User> {
	if (!user) {
		const result = await sendRequest<User>(getUserRequest);
		if (!result.ok) return result;
		user = result.value;
	}
	return Ok(user);
}
