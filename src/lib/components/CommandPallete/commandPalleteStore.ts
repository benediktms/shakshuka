import { writable } from 'svelte/store';

type CommandPalleteState = { open: boolean; value: string };

export const commandPallete = writable<CommandPalleteState>({ open: false, value: '' });
