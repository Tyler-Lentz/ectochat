<script lang="ts">
    import Message from "$lib/Message.svelte"
	import type { MessageData } from "./bindings/MessageData";

    let data: MessageData = {
        name: "Tyler",
        uid: BigInt(9210719247023),
        mid: BigInt(1908140349),
        timestamp: BigInt(1701221985),
        payload: [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33],
    };
    let data2: MessageData = {
        name: "John",
        uid: BigInt(9210719247023),
        mid: BigInt(1908140349),
        timestamp: BigInt(1701222985),
        payload: data.payload.concat(data.payload.concat(data.payload.concat(data.payload)))
    };
</script>

<main>
    <div>
        <Message
            data={data}
            >
        </Message>
        <Message
            data={data2}
            >
        </Message>
    </div>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        overflow-y: scroll;
    }

    main > * {
        padding: 1em 0em;
    }
</style>