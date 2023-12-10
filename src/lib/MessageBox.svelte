<script lang="ts">
    import type { MessageData } from "$lib/bindings/MessageData";
    import { modal_closed, profile } from "$lib/stores";
    import EyeIcon from '$lib/icons/eye.svg';
    import Canvas from '$lib/Canvas.svelte';
	import { PROFILE_PIC_SIZE } from "$lib/contants";
	import { onMount } from "svelte";
    import { openModal } from 'svelte-modals';
    import AckModal from '$lib/AckModal.svelte';
	import { writable, type Writable } from "svelte/store";

    let canvas: Canvas;
    export let data: MessageData;
    export let pic: number[]

    onMount(() => {
        if (pic.length > 0) {
            try {
                let imageData = new ImageData(
                    new Uint8ClampedArray(pic),
                    PROFILE_PIC_SIZE, 
                    PROFILE_PIC_SIZE 
                );
                canvas.setImageData(imageData);
            } catch(e) {
                console.error(e);
            }
        }
    });

    const message = data.payload.map((octet) => String.fromCharCode(octet)).join('');
    const date = new Date(Number(data.timestamp) * 1000)

    export let acks: {name: string, uid: string}[];

    let hovering: boolean = false;
    let clicked: boolean = false;
    let opened: boolean = false;
    let timeout_code: number;
    let startClose: Writable<boolean> = writable(false);
    function hoverAcks() {
        if (clicked) {
            return;
        }

        hovering = true;
        timeout_code = setTimeout(() => {
            // If still hovering in 250ms, then open the modal
            if ((hovering || clicked) && !clicked) {
                opened = true;
                openModal(AckModal, {message: acks, startClose})
                hovering = false;
            }
        }, 250)
    }

    function leaveHoverAcks() {
        if (!clicked) {
            hovering = false;
            opened = false;
            clearTimeout(timeout_code);
            startClose.set(true);
        }
    }

    function handleAckClick() {
        clicked = !clicked;
        clearTimeout(timeout_code);

        if (clicked) {
            if (!opened) {
                openModal(AckModal, {message: acks, startClose});
            }
        } else {
            startClose.set(true);
            opened = false;
        }
    }

    modal_closed.subscribe((was_closed) => {
        if (was_closed) {
            opened = false;
            clicked = false;

            modal_closed.set(false);
        }
    });

</script>

<article class="container">
    <aside id="timestamp">
        {date.toLocaleTimeString()}
    </aside> 
            <div class="canvas-container">
                <Canvas 
                    bind:this={canvas}
                    width={PROFILE_PIC_SIZE}
                    height={PROFILE_PIC_SIZE}
                    editable={false}
                    />
            </div>
    <section class="message-container {(data.uid == $profile?.uid) ? "from-self": "from-other"}">
        <header>
            <span id="name">{data.name}</span>
            <span id="uid">{data.uid.toString(16)}</span>
        </header>
        <textarea id="message">{message}</textarea>
    </section>
    <button class="ack-container" 
         data-num-acks={acks.length} 
         data-acks={acks}
         data-clicked={clicked}
         data-opened={opened}
         on:click={handleAckClick}
         on:mouseenter={hoverAcks}
         on:mouseleave={leaveHoverAcks}
         >
        <img id="ack" src={EyeIcon} alt="Acks"/>
    </button>
</article>

<style>
    .container {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        user-select: none;
        -webkit-user-select: none;
        width: 100%;
    }

    .message-container {
        border-radius: 4px;
        padding: 1rem;
        padding-bottom: 0;
        margin: 1rem;
        width: 60ch;
        background-color: var(--ctp-latte-mantle);
    }

    .message-container.from-self {
        border: 1px solid var(--ctp-latte-overlay1);
    }

    /* .message-container.from-other {
        
    } */

    .message-container header {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }
        
    #timestamp {
        color: var(--ctp-latte-overlay0);
        user-select: text;
        -webkit-user-select: text;
    }

    #name {
        color: var(--ctp-latte-blue);
        user-select: text;
        -webkit-user-select: text;
    }

    #uid {
        color: var(--ctp-latte-overlay0);
        user-select: text;
        -webkit-user-select: text;
    }

    #message {
        user-select: text;
        -webkit-user-select: text;

        padding-top: 1rem;
        text-align: left;
        display: inline-block;

        background-color: inherit;
        border: none;
        outline: 0;
        width: 100%;
        min-height: 1ch;
        max-height: 8ch;
        resize: none;
    }

    .canvas-container {
        align-self: flex-start;
        scale: 0.75;
        margin-top: 1.5em;
    }

    .ack-container {
        z-index: 100; /* so that when modal covers screen mouseleave event still tracks this element */
        outline: none;
        display: flex;
        flex-direction: row; /* make num appear to side */
        padding-right: 1em;
    }

    .ack-container:not([data-clicked="true"]) {
        /* set up underline transition*/
        transition: background-size 250ms;
        background: 
            linear-gradient(to right, transparent, transparent),
            linear-gradient(to right, var(--ctp-latte-blue), var(--ctp-latte-blue));
        background-size: 100% 0.1em, 0 0.1em;
        background-position: 100% 100%, 0 100%;
        background-repeat: no-repeat;
    }

    .ack-container[data-clicked="true"] {
        animation-name: pulse;
        animation-duration: 500ms;
        animation-iteration-count: 1;
        border-radius: 10px;
    }

    /* 
        adapted from https://codepen.io/olam/pen/KKMvWM 
        copied blue color from catpuccin style because could
        not figure out how to get the css variable inside
        of the rgba
    */
    @keyframes pulse {
        0% {
            box-shadow: 0 0 0 0 rgba(30,102,245, 0.4);
        }
        70% {
            box-shadow: 0 0 0 10px rgba(30,102,245, 0.0);
        }
        100% {
            box-shadow: 0 0 0 0 rgba(30,102,245, 0.0);
        }
    }

    .ack-container:hover:not([data-clicked="true"]) {
        background-size: 0 0.1em, 100% 0.1em;
    }

    .ack-container:after {
        position: relative;
        color: var(--ctp-latte-blue);
        right: -0.2ch;
        content: attr(data-num-acks);
        text-align: center;
    }

    .ack-container #ack {
        /* Make --ctp-latte-blue */
        filter: invert(39%) sepia(94%) saturate(4642%) hue-rotate(214deg) brightness(96%) contrast(100%);
    }

</style>