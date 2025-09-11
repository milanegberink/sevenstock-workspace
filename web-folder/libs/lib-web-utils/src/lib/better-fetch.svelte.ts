import { page } from '$app/state';

export function currentPage() {
	console.log(page.url);
}
