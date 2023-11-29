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
</script>

<Message
    data={data}
    >
</Message>

<style>

</style>