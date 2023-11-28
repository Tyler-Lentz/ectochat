<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { profile } from '$lib/stores';
    import type { Profile } from '$lib/bindings/Profile';

    let entered_name: String;

    function enterInfo() {
        invoke('cmd_set_profile_name', {name: entered_name})
            .then((resp: any) => {
                $profile = {
                    name: resp.name,
                    uid: resp.uid,
                    join_time: resp.uid,
                } as Profile;
            })
            .catch(err => {
                alert(err);
            })
    }
</script>

<h1>Ektochat</h1>

<form on:submit={enterInfo}>
    <input 
        type="text" 
        placeholder="Enter your name" 
        bind:value={entered_name}
        />

    <input 
        type="submit"
        >
</form>

<style>

</style>