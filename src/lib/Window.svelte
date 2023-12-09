<script lang="ts">
	import EnterScreen from "$lib/EnterScreen.svelte";
	import ChatScreen from "$lib/ChatScreen.svelte"
	// import { appWindow } from '@tauri-apps/api/window';
	import { profile } from '$lib/stores';

</script>

{#if $profile == null}
	<EnterScreen></EnterScreen>
{:else}
	<ChatScreen></ChatScreen>
{/if}