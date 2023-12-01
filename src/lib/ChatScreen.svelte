<script lang="ts">
    import MessageBox from "$lib/MessageBox.svelte"
	import type { MessageData } from "$lib/bindings/MessageData";
    import type { Message } from "$lib/bindings/Message";
	import { msg_history } from "$lib/stores";

    let data: MessageData = {
        name: "Tyler",
        uid: BigInt(8210719247023),
        mid: BigInt(1908140349),
        timestamp: BigInt(1701221985),
        payload: [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33],
    };

    let data2: MessageData = {
        name: "John",
        uid: BigInt(9210719247023),
        mid: BigInt(2908140349),
        timestamp: BigInt(1701222985),
        payload: data.payload.concat(data.payload.concat(data.payload.concat(data.payload)))
    };

    $msg_history = [
        {
            "Ack": {
                mid: BigInt(1908140349),
                uid: BigInt(9210719247023),
            }
        },
        {
            "Ack": {
                mid: BigInt(2908140349),
                uid: BigInt(8210719247023),
            }
        },
        {
            "Ack": {
                mid: BigInt(2908140349),
                uid: null
            }
        },
        {
            "Ack": {
                mid: BigInt(2908140349),
                uid: null
            }
        },
        {
            "Ack": {
                mid: BigInt(2908140349),
                uid: null
            }
        },
        {
            "Ack": {
                mid: BigInt(2908140349),
                uid: null
            }
        },
        {
            "Ack": {
                mid: BigInt(2908140349),
                uid: null
            }
        },
        {
            "Ack": {
                mid: BigInt(2908140349),
                uid: null
            }
        },
        {
            "Ack": {
                mid: BigInt(2908140349),
                uid: null
            }
        },
    ]
</script>

<main>
    <div>
        <MessageBox
            data={data}
            >
        </MessageBox>
        <MessageBox
            data={data2}
            >
        </MessageBox>
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