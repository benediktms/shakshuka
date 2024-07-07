import { writable } from 'svelte/store';

export const currentlyFocusedTaskId = writable<string | undefined>();
