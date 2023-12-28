<script lang="ts">
	import type { Writable } from 'svelte/store';
	import GenericModal from './GenericModal.svelte';
	import Canvas from './Canvas.svelte';
	import { MODAL_Z_INDEX, MESSAGE_PIC_HEIGHT, MESSAGE_PIC_WIDTH} from './contants';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';

    export let isOpen: boolean;
    export let startClose: Writable<boolean>;

    let canvas: Canvas;
    const PADDING: number = 32;

    function handleKeyPress(e: KeyboardEvent) {
        if ($startClose) {
            e.preventDefault();
            return;
        }
        
        switch (e.code) {
            case "Enter":
                sendImg(); 
                break;
        }
    }

    function sendImg() {
        let img = canvas.getFormattedImageData();

        invoke("cmd_send_img", {img})

        startClose.set(true);
    }

    let scale: number;

    onMount(() => {
        handleResize();
    });

    function handleResize() {
        if (window.matchMedia("(min-width: 1400px)").matches) {
            scale = 2.5;
        } else if (window.matchMedia("(min-width: 1100px)").matches) {
            scale = 2;
        } else {
            scale = 1.5;
        }
    }
</script>

<svelte:window on:keydown={handleKeyPress} on:resize={handleResize}/>

<GenericModal
    {isOpen}
    {startClose}
    modal_height={MESSAGE_PIC_HEIGHT + (PADDING * 2)}
    style={`padding: ${PADDING}px; scale: ${scale};`}
    --z-index={MODAL_Z_INDEX}
    >
    <div id="container">
        <Canvas 
            bind:this={canvas}
            width={MESSAGE_PIC_WIDTH}
            height={MESSAGE_PIC_HEIGHT}
            editable={true}
            />
        <div id="input-container">
            <input type="button" on:click={() => startClose.set(true)} value="Exit" />
            <input type="button" on:click={sendImg} value="Send" />
        </div>
    </div>
</GenericModal>

<style>
    input:is([type="button"], [type="submit"]) {
        scale: 0.5;
    }

    #input-container {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-around;
    }
</style>
