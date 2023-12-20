<script lang="ts">
	import { known_users } from "$lib/stores";

    let num_other_users = 0;
    known_users.subscribe((new_known_users) => {
        if (new_known_users == null) {
            num_other_users = 0;
        } else {
            num_other_users = Object.keys(new_known_users.uid_to_profile).length - 1;
        }
    });

</script>

<div id="container">
    <span>
        Chatting with <span class="highlight">{num_other_users}</span> {(num_other_users == 1) ? "user" : "users"}
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