export async function setToken() {
	if ('serviceWorker' in navigator) {
		const registration = await navigator.serviceWorker.ready;
		registration.active?.postMessage('SET_TOKEN');
	}
}

// export async function setUser() {
// 	if ('serviceWorker' in navigator) {
// 		const registration = await navigator.serviceWorker.ready;
// 		registration.active?.postMessage('GET_USER');
// 	}
// }
