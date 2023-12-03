<script lang="ts">
	import EnterScreen from "$lib/EnterScreen.svelte";
	import ChatScreen from "$lib/ChatScreen.svelte"
	// import { appWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api';
	import { profile } from '$lib/stores';

	profile.subscribe((val) => {
		if (val != null) {
			invoke('cmd_send_hello');
		}
	});
</script>

{#if $profile == null}
	<EnterScreen></EnterScreen>
{:else}
	<ChatScreen></ChatScreen>
{/if}