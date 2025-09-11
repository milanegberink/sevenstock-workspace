import { getContext, setContext } from 'svelte';

const FORM_CONTEXT_KEY = Symbol('FORM_CONTEXT');

export function setFormContext(context) {
	setContext(FORM_CONTEXT_KEY, context);
}

export function getFormContext() {
	return getContext(FORM_CONTEXT_KEY);
}
