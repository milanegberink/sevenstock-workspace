import { getContext, setContext } from 'svelte';

export const SIDEBAR_CONTEXT = Symbol('SIDEBAR_CONTEXT');

export function setSidebarContext(opened: () => boolean) {
	setContext(SIDEBAR_CONTEXT, opened);
}

export function getSidebarContext() {
	const ctx = getContext<() => boolean>(SIDEBAR_CONTEXT);
	return ctx ?? (() => true);
}
