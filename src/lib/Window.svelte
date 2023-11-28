<script lang="ts">
	import EnterScreen from "./EnterScreen.svelte";
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
	<h1>logged in, sending hello</h1>
{/if}

<style>

</style>