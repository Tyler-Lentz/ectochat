<script lang="ts">
	import type { MessageData } from "$lib/bindings/MessageData";
	import { onMount } from "svelte";
	import Canvas from "$lib/Canvas.svelte";
	import { PROFILE_PIC_SIZE } from "$lib/contants";

    // TODO: refactor this because i hate this exported interface 
    // and how rigid it is

    export let data: MessageData;
    export let msg1: string; // is followed by the name/uid found in data
    export let msg2: string; // follows the name.uid found in data

    let canvas: Canvas;
    onMount(() => {
        if (data.payload.length > 0) {
            try {
                let imageData = new ImageData(
                    new Uint8ClampedArray(data.payload),
                    PROFILE_PIC_SIZE, 
                    PROFILE_PIC_SIZE 
                );
                canvas.setImageData(imageData);
            } catch(e) {
                console.error(e);
            }
        }
    });
</script>

<div class="container">
    <p>
        {msg1}
        <span id="name">{data.name}</span> <span id="uid">({data.uid.toString(16)})</span>
        {msg2}
    </p>
    <Canvas 
        bind:this={canvas} 
        width={PROFILE_PIC_SIZE}
        height={PROFILE_PIC_SIZE}
        editable={false}
        />
</div>


<style>
    .container {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        text-align: center;
    }

    p {
        padding: 0;
        margin: 0;
    }

    #name {
        color: var(--ctp-latte-blue);
    }

    #uid {
        color: var(--ctp-latte-overlay0);
    }
</style>