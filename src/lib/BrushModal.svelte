<script lang="ts">
    // Slightly modified from svelte-modal documentation here:
    // https://www.npmjs.com/package/svelte-modals

	import type { Writable } from 'svelte/store';
	import GenericModal from './GenericModal.svelte';
	import Canvas from './Canvas.svelte';
	import { MESSAGE_PIC_SIZE } from './contants';
	import { appWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api';

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
</script>

<svelte:window on:keydown={handleKeyPress}/>

<GenericModal
    {isOpen}
    {startClose}
    modal_height={MESSAGE_PIC_SIZE + (PADDING * 2)}
    style={`padding: ${PADDING}px; scale: 2`}
    >
    <div id="container">
        <Canvas 
            bind:this={canvas}
            width={MESSAGE_PIC_SIZE}
            height={MESSAGE_PIC_SIZE}
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
