<script lang="ts">
    // Slightly modified from svelte-modal documentation here:
    // https://www.npmjs.com/package/svelte-modals

	import { writable, type Writable } from 'svelte/store';
	import GenericModal from './GenericModal.svelte';

    export let isOpen: boolean;

    export let startClose: Writable<boolean> = writable(false);
    export let message: string;
    export let result: Writable<string>;

    function handleSubmit(e: SubmitEvent) {
        result.set((e.submitter as HTMLInputElement)?.value);
        startClose.set(true);
    }
</script>

<GenericModal
    {isOpen}
    {startClose}
    modal_height={150}
    >
    <h2>{message}</h2>
    <form on:submit={handleSubmit}>
        <input type="submit" value="Yes"/>
        <input type="submit" value="No"/>
    </form>
</GenericModal>

<style>
    h2 {
        text-align: center;
        font-size: 24px;
    }

    form {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-around;
    }
</style>
