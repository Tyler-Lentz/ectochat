<script lang="ts">
    // Slightly modified from svelte-modal documentation here:
    // https://www.npmjs.com/package/svelte-modals

	import type { Writable } from 'svelte/store';
	import GenericModal from './GenericModal.svelte';

    export let isOpen: boolean;
    export let message: {name: string, uid: string}[];
    export let startClose: Writable<boolean>;

    let PIXELS_PER_ROW = 27;
    let modal_height: number = 100 + (PIXELS_PER_ROW * message.length);
</script>

<GenericModal
    {isOpen}
    {startClose}
    {modal_height}
    >
    <h2>Seen By</h2>
    <table>
        <thead>
            <tr>
                <th>Name</th>
                <th>UID</th>
            </tr>
        </thead>
        <tbody>
            {#each message as ack}
                <tr>
                    <td>{ack.name}</td>
                    <td>{ack.uid}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</GenericModal>

<style>
    h2 {
        text-align: center;
        font-size: 24px;
    }

    table {
        text-align: center;
        border-collapse: collapse;
    }

    tr {
        border: 1px solid var(--ctp-latte-overlay0);
    }

    th {
        color: var(--ctp-latte-blue);
    }
</style>
