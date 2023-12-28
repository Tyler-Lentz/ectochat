<script lang="ts">
	import { known_users } from "$lib/stores";
    import usersIcon from '$lib/icons/users.svg';
	import { openModal } from "svelte-modals";
	import KnownUsersModal from "./KnownUsersModal.svelte";
	import { writable } from "svelte/store";

    let num_other_users = 0;
    known_users.subscribe((new_known_users) => {
        if (new_known_users == null) {
            num_other_users = 0;
        } else {
            num_other_users = Object.keys(new_known_users.uid_to_profile).length - 1;
        }
    });

    function openKnownUsersModal() {
        if (num_other_users > 0) {
            openModal(KnownUsersModal, {startClose: writable(false) });
        }
    }

</script>

<div class="container">
    <span>
        Chatting with <span class="highlight" data-active={(num_other_users > 0)}>{num_other_users}</span> {(num_other_users == 1) ? "user" : "users"}
        <button class="icon-btn" data-active={(num_other_users > 0)} on:click={openKnownUsersModal} >
            <img src={usersIcon} alt="See Known Users"/>
        </button>
    </span>
</div>

<style>
    .container {
        padding: 0;
        margin: 0;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-around;
    }

    .container > span {
        display: flex;
        align-items: center;
    }

    .icon-btn {
        margin-left: 0.5em;
    }

    .icon-btn:not([data-active="true"]) {
        opacity: 0.5;
        background: none !important; /* override global hover style */
    }

    .highlight {
        padding: 0ch 0.5ch;
        color: var(--ctp-latte-overlay2);
    }

    .highlight[data-active="true"] {
        color: var(--ctp-latte-blue);
    }
</style>