<script lang="ts">
	import type { Writable } from 'svelte/store';
	import GenericModal from '$lib/GenericModal.svelte';
	import { onMount } from 'svelte';
	import type { Profile } from '$lib/bindings/Profile';
	import { invoke } from '@tauri-apps/api';
	import type { KnownUsers } from '$lib/bindings/KnownUsers';
    import arrowLeft from "$lib/icons/arrow_left.svg"
    import arrowRight from "$lib/icons/arrow_right.svg"
	import Canvas from './Canvas.svelte';
	import { PROFILE_PIC_SIZE } from './contants';

    export let isOpen: boolean;
    export let startClose: Writable<boolean>;

    let profiles: Profile[] = [];
    let index: number = 0;

    let profile: Profile | undefined;
    $: profile = profiles.at(index);

    onMount(() => {
        invoke("cmd_get_known_users")
            .then((known_users) => {
                profiles = Object.values((known_users as KnownUsers).uid_to_profile);
            })
    });

    function wrapIndex() {
        if (index < 0) {
            index = profiles.length - 1;
        }
        if (index >= profiles.length) {
            index = 0;
        }
    }

    function moveLeft() {
        index--;
        wrapIndex();
    }

    function moveRight() {
        index++;
        wrapIndex();
    }
</script>

<GenericModal
    {isOpen}
    {startClose}
    modal_height={400}
    >
    <div id="modal-container">
        <div id="profile-container">
            {#if profile !== undefined}
                <h2>{profile.name}</h2>
                <Canvas 
                    width={PROFILE_PIC_SIZE}
                    height={PROFILE_PIC_SIZE}
                    editable={false}
                    data={profile.pic}
                    />
            {/if}
        </div>
        <div id="controls-container">
            <button class="icon-btn" on:click={moveLeft} >
                <img src={arrowLeft} alt={"Left"} />
            </button>
            <button class="icon-btn" on:click={moveRight} >
                <img src={arrowRight} alt={"Right"} />
            </button>
        </div>
    </div>
</GenericModal>

<style>
    h2 {
        text-align: center;
        font-size: 24px;
    }

    #modal-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    #profile-container {
        margin-bottom: auto;
    }

    #controls-container {
        display: flex;
        flex-direction: row;
    }
</style>
