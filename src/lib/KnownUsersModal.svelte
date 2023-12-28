<script lang="ts">
	import type { Writable } from 'svelte/store';
	import GenericModal from '$lib/GenericModal.svelte';
	import { onMount } from 'svelte';
	import type { Profile } from '$lib/bindings/Profile';
	import { invoke } from '@tauri-apps/api';
	import type { KnownUsers } from '$lib/bindings/KnownUsers';
	import ProfileCard from '$lib/ProfileCard.svelte';
	import { profile } from '$lib/stores';

    export let isOpen: boolean;
    export let startClose: Writable<boolean>;

    let profiles: Profile[] = [];

    onMount(() => {
        invoke("cmd_get_known_users")
            .then((known_users) => {
                profiles = Object.values((known_users as KnownUsers).uid_to_profile);
            })
    });

</script>

<GenericModal
    {isOpen}
    {startClose}
    modal_height={400}
    >
    <div id="modal-container">
        <div id="profile-container">
            {#each profiles as curr_profile}
                {#if curr_profile.uid != $profile?.uid}
                    <ProfileCard profile={curr_profile} />
                {/if}
            {/each}
        </div>
    </div>
</GenericModal>

<style>
    #modal-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    #profile-container {
        height: 100%;
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
    }

</style>
