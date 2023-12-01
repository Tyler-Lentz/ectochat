<script lang="ts">
    import type { MessageData } from "$lib/bindings/MessageData";
    import type { Message } from "$lib/bindings/Message";
    import { msg_history, profile } from "$lib/stores";
    import WaveIcon from '$lib/icons/wave.svg';

    export let data: MessageData;

    const message = data.payload.map((octet) => String.fromCharCode(octet)).join('');
    const date = new Date(Number(data.timestamp) * 1000)

    let acks: string[];
    msg_history.subscribe((new_hist) => {
        // Pull out list of UIDS of users that have acked this Message
        // If User is in anonymous mode, it will be a string that says "Anonymous"
        // otherwise, it will be a hex string of the UID
        acks = new_hist
                .reduce((acked_uids, current) => {
                    if ("Ack" in current && current.Ack.mid == data.mid) {
                        if (current.Ack.uid == null) {
                            return acked_uids.concat("Anonymous");
                        } 
                        return acked_uids.concat("0x" + current.Ack.uid.toString(16));
                    }
                    return acked_uids;
                }, <string[]>[])
                
    })

    function clickAcks() {
        alert(acks)
    }

</script>

<article class="container">
    <aside id="timestamp">
        {date.toLocaleTimeString()}
    </aside> 
    <section class="message-container {(data.uid == $profile?.uid) ? "from-self": "from-other"}">
        <header>
            <span id="name">{data.name}</span>
            <span id="uid">0x{data.uid.toString(16)}</span>
        </header>
        <span id="message">{message}</span>
    </section>
    <button class="ack-container" 
         data-num-acks={acks.length} 
         data-acks={acks}
         on:click={clickAcks}
         >
        <img id="ack" src={WaveIcon} alt="Acks"/>
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
    }

    .ack-container {
        position: relative;
        right: 0;

        /* set up underline transition*/
        background: 
            linear-gradient(to right, var(--ctp-latte-base), var(--ctp-latte-base)),
            linear-gradient(to right, var(--ctp-latte-blue), var(--ctp-latte-blue));
        background-size: 100% 0.1em, 0 0.1em;
        background-position: 100% 100%, 0 100%;
        background-repeat: no-repeat;

        transition: background-size 400ms, right 0.25s ease-in-out;
    }

    .ack-container:hover {
        right: -0.33em;
        background-size: 0 0.1em, 100% 0.1em;

    }

    .ack-container:after {
        position: relative;
        color: var(--ctp-latte-blue);
        right: -0.2ch;
        content: attr(data-num-acks);
    }

    .ack-container #ack {
        /* Make --ctp-latte-blue */
        filter: invert(39%) sepia(94%) saturate(4642%) hue-rotate(214deg) brightness(96%) contrast(100%);
    }

    .ack-container:hover #ack {
        animation-name: wave-animation;
        animation-duration: 2.5s; 
        transform-origin: 50% 80%;
    }

    @keyframes wave-animation {
        /* https://codepen.io/jakejarvis/pen/pBZWZw */
        0% { transform: rotate( 0.0deg) }
        10% { transform: rotate(14.0deg) }  /* The following five values can be played with to make the waving more or less extreme */
        20% { transform: rotate(-8.0deg) }
        30% { transform: rotate(14.0deg) }
        40% { transform: rotate(-4.0deg) }
        50% { transform: rotate(10.0deg) }
        60% { transform: rotate( 0.0deg) }  /* Reset for the last half to pause */
        100% { transform: rotate( 0.0deg) }
    }
</style>