import { writable } from 'svelte/store';
import type {Writable} from 'svelte/store';
import type { Message } from '$lib/bindings/Message';
import type { Profile } from '$lib/bindings/Profile';

export const msg_history: Writable<Array<Message>> = writable([]);
export const profile: Writable<Profile | null> = writable(null);

// This will be set to true when a modal is closed via the esc key or clicking
// outside the modal itself. Then, the component that is handling the modal
// can know when the modal is closed and do appropriate styling/variable
// tracking. The listener must set it back to false for the next time;
// Currently set to true in +page.svelte, and listened for in MessageBox.svelte.
export const modal_closed: Writable<boolean> = writable(false);