<script lang="ts">
	import EnterScreen from "$lib/EnterScreen.svelte";
	import ChatScreen from "$lib/ChatScreen.svelte"
	import { appWindow } from '@tauri-apps/api/window';
	import { msg_history } from '$lib/stores';
	import type { Message } from '$lib/bindings/Message';

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
</script>

{#if !initialized}
	<EnterScreen on:initialized={initialize}></EnterScreen>
{:else}
	<ChatScreen></ChatScreen>
{/if}