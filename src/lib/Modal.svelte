<script lang="ts">
	import { onMount } from 'svelte';
    import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

    // Slightly modified from svelte-modal documentation here:
    // https://www.npmjs.com/package/svelte-modals

    import { closeModal } from 'svelte-modals'
	import type { Writable } from 'svelte/store';

    export let isOpen: boolean;

    export let title: string;
    export let message: string;

    export let startClose: Writable<boolean>;
    let local: boolean = false;

    let contents: HTMLDivElement;
    startClose.subscribe((new_val) => {
        if (new_val) {
            local = true;

            setTimeout(() => {
                startClose.set(false);
                local = false;
                closeModal();
            }, 500);
        } 
    });

</script>

{#if isOpen}
<div role="dialog" class="modal">
    <div bind:this={contents} class="contents" data-open={isOpen} data-close={local}>
        <h2>{title}</h2>
        <p>{message}</p>
        <div class="actions">
            <button on:click="{closeModal}">OK</button>
        </div>
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

    .contents {
        min-width: 240px;
        border-radius: 6px;
        padding: 16px;
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
        height: 200px;
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
            height: 200px;
        }
    }

    @keyframes slideClosed {
        0% {
            height: 200px;
        }

        5% {
            height: 200px
        }

        100% {
            height: 0px;
        }
    }

    h2 {
        text-align: center;
        font-size: 24px;
    }

    p {
        text-align: center;
        margin-top: 16px;
    }

    .actions {
        margin-top: 32px;
        display: flex;
        justify-content: flex-end;
    }

    .actions > button:hover {
        background-color: var(--ctp-latte-rosewater);
    }
</style>
