<script lang="ts">
    import { profile } from '$lib/stores';
	import type { Message } from '$lib/bindings/Message';

    import { invoke } from '@tauri-apps/api'
	import Canvas from '$lib/Canvas.svelte';
	import { onMount } from 'svelte';
	import { PROFILE_PIC_SIZE } from '$lib/contants';

    let current_time = new Date();
    setInterval(() => {
        current_time = new Date();
    }, 1000)


    let canvas: Canvas;
    onMount(() => {
        let imageData = new ImageData(
            new Uint8ClampedArray($profile?.pic || []),
            PROFILE_PIC_SIZE, 
            PROFILE_PIC_SIZE 
        );
        canvas.setImageData(imageData);
    });

    let message_str: string;
    function checkForEnter(e: KeyboardEvent) {
        if (e.code == "Enter") {
            e.preventDefault();
            if (message_str.length > 0) {
                invoke('cmd_send_text', {msg: message_str});

                message_str = '';
            }
        }
    }

</script>

<form id="container" >
    <div id="canvas-container">
        <Canvas 
            bind:this={canvas}
            width={PROFILE_PIC_SIZE}
            height={PROFILE_PIC_SIZE}
            editable={false}
            color={'black'}
            />
    </div>

    <div id="message-container">
        <header>
            <span id="name">{$profile?.name}</span>
            <span id="uid">0x{$profile?.uid.toString(16)}</span>
        </header>
        <textarea 
            wrap="soft"
            placeholder="Enter message here"
            bind:value={message_str}
            on:keypress={checkForEnter}
            />
    </div>
</form>

<style>
    #container {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        margin: auto;
        width: 95%;
        height: 100%;
    }

    #canvas-container {
        margin-right: auto;
        display: flex;
        scale: 0.75;
    }

    #message-container {
        background-color: var(--ctp-latte-base);
        border-radius: 4px;
        border: 1px solid var(--ctp-latte-overlay1);
        padding: 1em;
        margin: 1em;

        width: min(100%, 60ch);

        display: flex;
        flex-direction: column;
        margin-right: auto;
        align-items: center;
        user-select: text;
        -webkit-user-select: text;
    }

    #message-container > header {
        width: 100%;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    #name {
        color: var(--ctp-latte-blue);
    }

    #uid {
        color: var(--ctp-latte-overlay0);
    }

    textarea {
        background-color: inherit;
        border: none;
        outline: 0;
        width: 100%;
        min-height: 4ch;
        margin: 1rem;
        padding: 1rem;
        resize: none;
    }

</style>