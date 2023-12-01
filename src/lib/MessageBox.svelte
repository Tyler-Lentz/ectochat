<script lang="ts">
    import type { MessageData } from "$lib/bindings/MessageData";
    import type { Message } from "$lib/bindings/Message";
    import { msg_history } from "$lib/stores";
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
    <section class="message-container">
        <header>
            <span id="name">{data.name}</span>
            <span id="uid">0x{data.uid.toString(16)}</span>
        </header>
        <main>
            <span id="message">{message}</span>
        </main>
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


        user-select: none;
        -webkit-user-select: none;
    }

    .message-container {
        border-radius: 4px;
        padding: 1rem;
        margin: 1rem;
        width: 60vh;
        background-color: var(--ctp-latte-mantle);

    }

    .message-container header {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }
        
    .message-container main {
        padding-top: 1rem;
        text-align: center;
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
    }

    .ack-container {
        position: relative;
        top: 0;

        /* set up underline transition*/
        background: 
            linear-gradient(to right, var(--ctp-latte-base), var(--ctp-latte-base)),
            linear-gradient(to right, var(--ctp-latte-blue), var(--ctp-latte-blue));
        background-size: 100% 0.1em, 0 0.1em;
        background-position: 100% 100%, 0 100%;
        background-repeat: no-repeat;

        transition: background-size 400ms, top 0.25s ease-in-out;
    }

    .ack-container:hover {
        top: -0.33em;
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


</style>