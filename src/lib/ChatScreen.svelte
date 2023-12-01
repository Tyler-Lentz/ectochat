<script lang="ts">
    import MessageBox from "$lib/MessageBox.svelte"
	import type { MessageData } from "$lib/bindings/MessageData";
    import type { Message } from "$lib/bindings/Message";
	import { msg_history, profile } from "$lib/stores";
	import InputBox from "./InputBox.svelte";

    let data: MessageData = {
        name: "Tyler",
        uid: ($profile?.uid) ? $profile.uid : BigInt(0),
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
    <section id="rec-messages">
        <div>
            <MessageBox
                data={data}
                >
            </MessageBox>
        </div>
        <div>
            <MessageBox
                data={data2}
                >
            </MessageBox>
        </div>
        <div>
            <MessageBox
                data={data2}
                >
            </MessageBox>
        </div>
        <div>
            <MessageBox
                data={data2}
                >
            </MessageBox>
        </div>
        <div>
            <MessageBox
                data={data2}
                >
            </MessageBox>
        </div>
        <div>
            <MessageBox
                data={data2}
                >
            </MessageBox>
        </div>
        <div>
            <MessageBox
                data={data2}
                >
            </MessageBox>
        </div>
        <div>
            <MessageBox
                data={data2}
                >
            </MessageBox>
        </div>
        <div>
            <MessageBox
                data={data2}
                >
            </MessageBox>
        </div>
    </section>
    <section id="input-message">
        <InputBox>
        </InputBox>
    </section>
</main>

<style>
    main {
        height: 100vh;
        width: 100%;

        display: flex;
        flex-direction: column;

        user-select: none;
        -webkit-user-select: none;
    }

    #rec-messages {
        width: 100%;
        height: 85vh;

        display: flex;
        flex-direction: column;
        align-items: center;
        overflow-y: scroll;
    }

    #rec-messages > * {
        width: 80vw;
    }

    #input-message {
        width: 100%;
        height: 15vh;

        border-top: 1px solid var(--ctp-latte-overlay0);

        background-color: var(--ctp-latte-mantle);
    }
</style>