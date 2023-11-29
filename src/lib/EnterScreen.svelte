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

<div class="container">
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
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        margin: auto;
    }

    h1 {
        font-size: 4rem;
    }

    form {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }


    input[type="text"] {
        padding: 1rem;
        margin: 1rem;
        text-align: center;
    }
</style>