<script lang="ts">
	import { onMount } from 'svelte';
    // Slightly modified from svelte-modal documentation here:
    // https://www.npmjs.com/package/svelte-modals

    // TODO: rename to AckModal.svelte
    // and pass in ack information

    import { closeModal } from 'svelte-modals'
	import type { Writable } from 'svelte/store';

    export let isOpen: boolean;

    export let message: [string,string][];

    export let startClose: Writable<boolean>;

    startClose.subscribe((new_val) => {
        if (new_val) {
            setTimeout(() => {
                startClose.set(false);
                closeModal();
            }, 500);
        } 
    });

    let PIXELS_PER_ROW = 27;
    let modal_height: number = 100 + (PIXELS_PER_ROW * message.length);

    onMount(() => {
        // let num_users = Math.floor(Math.random() * 10);
        let num_users = 20;

        message = new Array(num_users);
        modal_height = 100 + (PIXELS_PER_ROW * message.length);
        for (let i = 0; i < num_users; i++) {
            message[i] = ["Testlx", Math.floor(Math.random() * 1000000).toString(16)];
        }
    });
</script>

{#if isOpen}
<div role="dialog" class="modal" style="--modal-height: {modal_height}px;">
    <div class="contents" data-open={isOpen} data-close={$startClose}>
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
                        <td>{ack[0]}</td>
                        <td>{ack[1]}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</div>
{/if}

<style>
    .modal {
        position: fixed;
        top: 0;
        bottom: 0;
        right: 0;
        left: 0;
        display: flex;
        justify-content: center;
        align-items: center;

        /* allow click-through to backdrop */
        pointer-events: none;
    }

    .contents::-webkit-scrollbar {
        display: none;
    }

    .contents {
        overflow-y: scroll;
        min-width: 240px;
        max-height: 80vh;
        border-radius: 6px;
        padding: 16px;
        padding-top: 0px;
        background: white;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        pointer-events: auto;
    }

    .contents[data-open="true"] {
        animation-name: slideOpen;
        animation-iteration-count: 1;
        animation-duration: 0.5s;
        animation-timing-function: cubic-bezier(0.645, 0.045, 0.355, 1.000);
        height: var(--modal-height);
    }
    
    .contents[data-open="true"] > * {
        animation-name: appearAtEnd;
        animation-iteration-count: 1;
        animation-duration: 0.6s;
        animation-timing-function: linear;
    }

    .contents[data-close="true"] {
        animation-name: slideClosed;
        animation-iteration-count: 1;
        animation-duration: 0.5s;
        animation-timing-function: cubic-bezier(0.645, 0.045, 0.355, 1.000);
        height: 0px;
    }

    .contents[data-close="true"] > * {
        animation-name: disappearAtBegin;
        animation-iteration-count: 1;
        animation-duration: 0.5s;
        animation-timing-function: linear;
        opacity: 0;
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

    @keyframes appearAtEnd {
        0% {
            opacity: 0;
        }

        95% {
            opacity: 0;
        }

        100% {
            opacity: 1;
        }
    }

    @keyframes disappearAtBegin {
        0% {
            opacity: 1;
        }

        5% {
            opacity: 0;
        }

        100% {
            opacity: 0;
        }
    }

    @keyframes slideOpen {
        0% {
            height: 0;
        }

        100% {
            height: var(--modal-height);
        }
    }

    @keyframes slideClosed {
        0% {
            height: var(--modal-height);
        }

        5% {
            height: var(--modal-height);
        }

        100% {
            height: 0px;
        }
    }

    h2 {
        text-align: center;
        font-size: 24px;
    }
</style>
