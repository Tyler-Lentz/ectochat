import { writable } from 'svelte/store';
import type {Writable} from 'svelte/store';
import type { Message } from '$lib/bindings/Message';
import type { Profile } from '$lib/bindings/Profile';

export const msg_history: Writable<Array<Message>> = writable([]);
export const profile: Writable<Profile | null> = writable(null);