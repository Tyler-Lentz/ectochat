<script lang="ts">
    import Canvas from "$lib/Canvas.svelte";
    import Window from "$lib/Window.svelte"
    import { Modals, closeModal } from 'svelte-modals'

    function handleModalKeypress(e: KeyboardEvent) {
        if (e.key == "Escape") {
            closeModal();
        }
    }
</script>

<Window></Window>
<Modals>
    <div
        slot="backdrop"
        class="backdrop"
        role="button"
        tabindex=0
        on:click={closeModal}
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
</style>