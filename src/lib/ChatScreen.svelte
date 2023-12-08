<script lang="ts">
    import MessageBox from "$lib/MessageBox.svelte"
    import type { Message } from "$lib/bindings/Message";
	import { msg_history, profile } from "$lib/stores";
	import InputBox from "$lib/InputBox.svelte";
    import { appWindow, currentMonitor } from "@tauri-apps/api/window";
    import { invoke } from "@tauri-apps/api";
    import type { KnownUsers } from "$lib/bindings/KnownUsers";

    let rec_messages: HTMLElement;

    function getMsgUid(m: Message | undefined) {
        if (m == undefined) return 0;

        if ("Ack" in m) {
            return m.Ack.uid;
        } else if ("Text" in m) {
            return m.Text.uid;
        } else if ("Hello" in m) {
            return m.Hello.uid;
        } else if ("Image" in m) {
            return m.Image.uid;
        } else {
            return 0;
        }
    }

    appWindow.listen("evt_new_msg", (e) => {
        let msg = e.payload as Message;
        console.log("evt new msg", msg)

        msg_history.update(hist => {
            return [...hist, msg];
        });

        let scrolled_to_bottom = rec_messages.scrollTop + rec_messages.clientHeight >= rec_messages.scrollHeight;
        let from_self = getMsgUid($msg_history.at(-1)) == $profile?.uid;

        if (scrolled_to_bottom || from_self) {
            setTimeout(() => {
                rec_messages.scrollTo({top: rec_messages.scrollHeight, behavior: "smooth"});
            }, 0)
        }

    });

    // Map from MID (message id) to the formatted information about all the acks
    let mid_to_acks: Map<
        number,
        {name: string, uid: string}[]
    > = new Map();

    msg_history.subscribe((new_hist) => {
        // Pull out list of UIDS of users that have acked this Message
        // If User is in anonymous mode, it will be a string that says "Anonymous"
        // otherwise, it will be a hex string of the UID
        invoke("cmd_get_known_users")
            .then((payload: any) => {
                let known_users = payload as KnownUsers;

                mid_to_acks = new Map();

                new_hist.forEach((msg) => {
                    if ("Ack" in msg) { //&& msg.Ack.uid != $profile?.uid) {
                        let curr_ack_list = mid_to_acks.get(msg.Ack.mid);
                        if (curr_ack_list == undefined) {
                            mid_to_acks.set(msg.Ack.mid, []);
                            curr_ack_list = [];
                        }

                        if (msg.Ack.uid == null) {
                            mid_to_acks.set(
                                msg.Ack.mid,
                                curr_ack_list.concat({name: "Anonymous", uid: "N/A"})
                            );    
                        }  else {
                            const name = known_users.uid_to_name[msg.Ack.uid];
                            mid_to_acks.set(
                                msg.Ack.mid, 
                                curr_ack_list.concat({name: name, uid: msg.Ack.uid.toString(16)})
                            );
                        }
                    }
                });
            });
    })

</script>

<main>
    <section id="rec-messages" bind:this={rec_messages}>
        {#each $msg_history as msg}
            {#if "Hello" in msg}
                <div>
                    <MessageBox 
                        data={msg.Hello} 
                        acks={mid_to_acks.get(msg.Hello.mid) || []} 
                        />
                </div>
            {:else if "Text" in msg}
                <div>
                    <MessageBox
                        data={msg.Text}
                        acks={mid_to_acks.get(msg.Text.mid) || []} 
                        />
                </div>
            {/if}
        {/each}
    </section>
    <section id="input-message">
        <InputBox />
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
        height: max(15vh, fit-content);

        border-top: 1px solid var(--ctp-latte-overlay0);

        background-color: var(--ctp-latte-mantle);
    }
</style>