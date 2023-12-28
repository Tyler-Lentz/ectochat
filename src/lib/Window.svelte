<script lang="ts">
	import EnterScreen from "$lib/EnterScreen.svelte";
	import ChatScreen from "$lib/ChatScreen.svelte"
	import { appWindow } from '@tauri-apps/api/window';
	import { known_users, msg_history } from '$lib/stores';
	import type { Message } from '$lib/bindings/Message';
	import type { KnownUsers } from "./bindings/KnownUsers";
	import Popup from "./Popup.svelte";

	let initialized = false;	

	function initialize() {
		initialized = true;
	}

    appWindow.listen("evt_new_msg", (e) => {
        let msg = e.payload as Message;

        msg_history.update(hist => {
            return [...hist, msg];
        });
	});

    appWindow.listen("evt_known_users_changed", (e) => {
        $known_users = e.payload as KnownUsers;
    })
</script>

{#if !initialized}
	<EnterScreen on:initialized={initialize}></EnterScreen>
{:else}
	<ChatScreen></ChatScreen>
{/if}