<script lang="ts">
    import type { MessageData } from "$lib/bindings/MessageData";

    export let data: MessageData;

    const message = data.payload.map((octet) => String.fromCharCode(octet)).join('');

    const date = new Date(Number(data.timestamp) * 1000)

</script>

<article class="container">
    <section class="time-container">
        <aside id="timestamp">
            {date.toLocaleTimeString()}
        </aside> 
    </section>
    <section class="message-container">
        <header>
            <span id="name">{data.name}</span>
            <span id="uid">0x{data.uid.toString(16)}</span>
        </header>
        <main>
            <span id="message">{message}</span>
        </main>
    </section>
</article>

<style>
    .container {
        display: flex;
        flex-direction: row;
        align-items: center;


        user-select: none;
        -webkit-user-select: none;
    }

    .time-container {
        color: var(--ctp-latte-overlay0);
    }

    .message-container {
        border-radius: 4px;
        padding: 1rem;
        margin: 1rem;
        width: 60vh;
        background-color: var(--ctp-latte-mantle);
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

    #timestamp {
        user-select: text;
        -webkit-user-select: text;
    }

    #message {
        user-select: text;
        -webkit-user-select: text;
    }

    header {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    main {
        padding-top: 1rem;
        text-align: center;
    }
</style>