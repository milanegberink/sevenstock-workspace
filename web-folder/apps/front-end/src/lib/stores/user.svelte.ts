import { goto } from '$app/navigation';
import { sendMessageToSw, SwRequest } from '$lib/service-worker/send-message';
import { Err, Ok, type PromiseResult } from '@lib/core';
import { type User } from '@lib/core/schemas';

let user: User | null = $state(null);

export async function getUser(): PromiseResult<User> {
	if (!user) {
		const result = await sendMessageToSw<User>(SwRequest.GetUser);
		if (!result.ok) return result;
		user = result.value;
	}
	return Ok(user);
}
