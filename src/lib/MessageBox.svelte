<script lang="ts">
    import type { MessageData } from "$lib/bindings/MessageData";
    import type { Message } from "$lib/bindings/Message";
    import { msg_history, profile } from "$lib/stores";
    import EyeIcon from '$lib/icons/eye.svg';
    import Canvas from '$lib/Canvas.svelte';
	import { PROFILE_PIC_SIZE } from "$lib/contants";
	import { onMount } from "svelte";
    import { openModal, closeModal } from 'svelte-modals';
    import Modal from '$lib/Modal.svelte';
	import { writable, type Writable } from "svelte/store";


    let canvas: Canvas;
    export let data: MessageData;

    onMount(() => {
        let imageData = new ImageData(
            new Uint8ClampedArray(data.pic),
            PROFILE_PIC_SIZE, 
            PROFILE_PIC_SIZE 
        );
        canvas.setImageData(imageData);
    });

    const message = data.payload.map((octet) => String.fromCharCode(octet)).join('');
    const date = new Date(Number(data.timestamp) * 1000)

    let acks: (bigint | null)[];
    msg_history.subscribe((new_hist) => {
        // Pull out list of UIDS of users that have acked this Message
        // If User is in anonymous mode, it will be a string that says "Anonymous"
        // otherwise, it will be a hex string of the UID
        // TODO: pull out acks in ChatScreen component, then pass down into individual
        // MessageBoxes so that this computation isn't repeated for each message box 
        acks = new_hist
                .reduce((acked_uids, current) => {
                    if ("Ack" in current && current.Ack.mid == data.mid ) { //&& current.Ack.uid != $profile?.uid) {
                        if (current.Ack.uid == null) {
                            return acked_uids.concat(null);
                        } 
                        return acked_uids.concat(current.Ack.uid);
                    }
                    return acked_uids;
                }, <(bigint | null)[]>[])
    })

    let hovering: boolean = false;
    let timeout_code: number;
    let startClose: Writable<boolean> = writable(false);
    function hoverAcks() {
        hovering = true;
        timeout_code = setTimeout(() => {
            // If still hovering in 500ms, then open the modal
            if (hovering) {
                openModal(Modal, {title: "Seen by", message: acks.join('\n'), startClose})
                hovering = false;
            }
        }, 500)
    }

    function leaveHoverAcks() {
        hovering = false;
        clearTimeout(timeout_code);
        startClose.set(true);
    }

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
        /* set up underline transition*/
        background: 
            linear-gradient(to right, var(--ctp-latte-base), var(--ctp-latte-base)),
            linear-gradient(to right, var(--ctp-latte-blue), var(--ctp-latte-blue));
        background-size: 100% 0.1em, 0 0.1em;
        background-position: 100% 100%, 0 100%;
        background-repeat: no-repeat;
        z-index: 100; /* so that when modal covers screen mouseleave event still tracks this element */

        transition: background-size 500ms;

        display: flex;
        flex-direction: row; /* make num appear to side */
    }

    .ack-container:hover {
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