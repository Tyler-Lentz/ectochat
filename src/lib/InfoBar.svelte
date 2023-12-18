<script lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	import type { KnownUsers } from "$lib/bindings/KnownUsers";

    let known_users: KnownUsers | null = null;

    appWindow.listen("evt_new_msg", (e) => {
        known_users = e.payload as KnownUsers;
    })

    function getNumOtherUsers() {
        if (known_users == null) {
            return 0;
        } else {
            return Object.keys(known_users.uid_to_profile).length - 1;
        }
    }

    function isThereOnlyOneOtherUser() {
        return getNumOtherUsers() == 1;
    }
</script>

<div id="container">
    <span>
        Chatting with <span class="highlight">{getNumOtherUsers()}</span> {(isThereOnlyOneOtherUser()) ? "user" : "users"}
    </span>
</div>

<style>
    #container {
        padding: 0;
        margin: 0;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-around;
    }

    .highlight {
        color: var(--ctp-latte-blue);
    }
</style>