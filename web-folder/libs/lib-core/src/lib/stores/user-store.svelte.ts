import { type PromiseResult, Err, Ok } from '$lib/result.js';
import type { User } from '$lib/schemas/user.js';
import { sendRequest } from '$lib/service-worker/make-request.js';
import { getUserRequest, setTokenRequest } from '$lib/service-worker/request.js';

function createUserStore() {
	let user = $state<User>();

	return {
		get user(): User | undefined {
			return user;
		},
		set user(u: User) {
			user = u;
		}
	};
}

export const userStore = createUserStore();

export async function initUser(): PromiseResult<void> {
	const result = await sendRequest<User>(setTokenRequest);

	if (!result.ok) {
		return result;
	}

	userStore.user = result.value;

	return Ok(undefined);
}
