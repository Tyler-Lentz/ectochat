<script lang="ts">
	import type { Profile } from "$lib/bindings/Profile";
	import Canvas from "./Canvas.svelte";
	import MessageBox from "./MessageBox.svelte";
	import { PROFILE_PIC_SIZE } from "./contants";

    export let profile: Profile;
    const timestamp = new Date(Number(profile.join_time) * 1000).toLocaleString();
</script>

<div class="container vertical">
    <div class="horizontal">
        <Canvas
            editable={false}
            width={PROFILE_PIC_SIZE}
            height={PROFILE_PIC_SIZE}
            data={profile.pic}
            />
        <div class="vertical">
            <span class="name">{profile.name}</span>
            <span class="uid">{profile.uid.toString(16)}</span>
        </div>
    </div>
    <span class="timestamp">{timestamp}</span>
</div>

<style>
    .container {
        user-select: none;
        -webkit-user-select: none;
        border: 1px solid var(--ctp-latte-overlay1);
        margin: 1em;
    }

    :is(.name, .uid, .timestamp) {
        user-select: text;
        -webkit-user-select: text;
    }

    .name {
        color: var(--ctp-latte-blue);
    }

    .uid {
        color: var(--ctp-latte-overlay0);
    }

    .timestamp {
        color: var(--ctp-latte-overlay2);
    }

    :is(.vertical, .horizontal) {
        display: flex;
        justify-content: center;
        align-items: flex-start;
    }

    .vertical {
        flex-direction: column;
    }

    .horizontal {
        flex-direction: row;
    }

    :is(.horizontal, .vertical) > * {
        margin: 0.5em;
    }
</style>