<script lang="ts">
    import MessageBox from "$lib/MessageBox.svelte"
	import type { MessageData } from "$lib/bindings/MessageData";
    import type { Message } from "$lib/bindings/Message";
	import { msg_history, profile } from "$lib/stores";
	import InputBox from "./InputBox.svelte";
    import { appWindow } from "@tauri-apps/api/window";

    appWindow.listen("evt_new_msg", (e) => {
        let msg = e.payload as Message;
        if ("Text" in msg) {
            msg_history.update(hist => [...hist, msg]);
        } else {
            // TODO: other types of messages
        }
    });

</script>

<main>
    <section id="rec-messages">
        {#each $msg_history as msg}
            {#if "Hello" in msg}
                <div>
                    <MessageBox data={msg.Hello} />
                </div>
            {:else if "Text" in msg}
                <div>
                    <MessageBox data={msg.Text} />
                </div>
            {/if}
        {/each}
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