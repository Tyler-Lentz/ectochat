<script lang="ts">
    import { profile } from '$lib/stores';
	import type { Message } from '$lib/bindings/Message';

    import { invoke } from '@tauri-apps/api'

    let current_time = new Date();
    setInterval(() => {
        current_time = new Date();
    }, 1000)

    let message_str: string;

    function handleSend(e: Event) {
        e.preventDefault();

        if (message_str.length > 0) {
            invoke('cmd_send_text', {msg: message_str});

            message_str = '';
        }
    }
</script>

<form id="container" on:submit={handleSend}>
    <div id="metadata">
        <span id="timestamp">{current_time.toLocaleTimeString()}</span>
        <span id="name">{$profile?.name}</span>
    </div>
    <input type="text" 
           placeholder="Enter message here"
           bind:value={message_str}
           />
</form>

<style>
    #container {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        margin: auto;
        width: 95%;
        height: 100%;
    }

    #metadata {
        display: flex;
        flex-direction: column;
        align-items: flex-start;

        user-select: text;
        -webkit-user-select: text;
    }

    #name {
        color: var(--ctp-latte-blue);
    }

    input {
        width: 60ch;
        margin: 1rem;
        padding: 1rem;
    }
</style>