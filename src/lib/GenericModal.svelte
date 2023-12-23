<script lang="ts">
    import { closeModal } from 'svelte-modals'
	import type { Writable } from 'svelte/store';

    export let modal_height: number;
    export let isOpen: boolean;
    export let startClose: Writable<boolean>;
    export let style: string = "";

    startClose.subscribe((new_val) => {
        if (new_val) {
            setTimeout(() => {
                startClose.set(false);
                closeModal();
            }, 500);
        } 
    });
</script>

{#if isOpen}
<div role="dialog" class="modal" style="--modal-height: {modal_height}px;">
    <div class="contents" style={style} data-open={isOpen} data-close={$startClose}>
        <slot />
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
    
    .contents[data-open="true"] > :global(*) {
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

    .contents[data-close="true"] > :global(*) {
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
</style>