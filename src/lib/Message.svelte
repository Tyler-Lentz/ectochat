<script lang="ts">
    import type { MessageData } from "$lib/bindings/MessageData";

    export let data: MessageData;

    const message = data.payload.map((octet) => String.fromCharCode(octet)).join('');

    const date = new Date(Number(data.timestamp))

</script>

<div class="container">
    <header>
        <span>{data.name}</span>
        <span>{date}</span>
        <span>{data.uid}</span>
    </header>
    <main>
        <span>{message}</span>
    </main>
</div>

<style>
    .container {
        border-radius: 20px;
        width: 66vw;

        display: flex;
        flex-direction: column;

        padding: 1rem;

        background-color: gray;
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