<script lang="ts">
	import EnterScreen from "./EnterScreen.svelte";
	// import { appWindow } from '@tauri-apps/api/window';
	import { profile } from '$lib/stores';
</script>

{#if $profile == null}
	<EnterScreen></EnterScreen>
{:else}
	<h1>logged in</h1>
{/if}

<style>

</style>