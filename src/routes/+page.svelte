<script lang="ts">
    import Canvas from "$lib/Canvas.svelte";
    import Window from "$lib/Window.svelte"
    import { Modals, closeModal } from 'svelte-modals'
    import { modal_closed } from "$lib/stores";

    function handleModalClose() {
        modal_closed.set(true);
        closeModal();
    }

    function handleModalKeypress(e: KeyboardEvent) {
        if (e.key == "Escape") {
            handleModalClose();
        }
    }
</script>

<svelte:window on:keydown={handleModalKeypress} />
<Window></Window>
<Modals>
    <div
        slot="backdrop"
        class="backdrop"
        role="button"
        tabindex=0
        on:click={handleModalClose}
        on:keydown={handleModalKeypress}
    />
</Modals>

<style>
    .backdrop {
        position: fixed;
        top: 0;
        bottom: 0;
        right: 0;
        left: 0;
        background-color: var(--ctp-latte-overlay0);
        opacity: 0.5;
    }

    :global(.icon-btn) {
        width: fit-content;
        background-color: transparent;
        border-radius: 10px;
        transition: 100ms background-color ease-in-out;
        outline: none;
    }

    :global(.icon-btn:hover) {
        background-color: var(--ctp-latte-overlay0);
    }
</style>