<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { profile } from '$lib/stores';
    import type { Profile } from '$lib/bindings/Profile';
    import Canvas from '$lib/Canvas.svelte';
    import { PROFILE_PIC_SIZE } from '$lib/contants';

    let entered_name: String;
    let canvas: Canvas;

    function enterInfo() {
        if (entered_name.length == 0) {
            // TODO: switch to in app modal
            alert('You must enter in a name with length > 0');
            return;
        }

        let profile_pic_data = (canvas.getImageData()?.data || []).toString();

        invoke('cmd_personalize_new_profile', {newName: entered_name, newPic: profile_pic_data})
            .then((r: any) => {
                let resp = r as Profile;
                $profile = {
                    name: resp.name,
                    uid: resp.uid,
                    join_time: resp.join_time,
                    pic: resp.pic,
                } as Profile;
            })
            .catch(err => {
                alert(err);
            })
    }

    let state: number = 0; 
</script>

<div class="container">
    <h1>ectochat</h1>

    {#if state == 0}
        <input 
            type="text" 
            placeholder="Enter your name" 
            bind:value={entered_name}
            />
    {:else if state == 1}
        <div id="canvas-container">
            <Canvas
                bind:this={canvas}
                width={PROFILE_PIC_SIZE}
                height={PROFILE_PIC_SIZE}
                editable={true}
                />
        </div>
    {:else if state == 2}
        <input 
            type="submit"
            value="Start Chatting"
            >
    {/if}
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
        text-align: center;
    }

    form {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }


    input:is([type="text"], [type="submit"]) {
        padding: 1rem;
        margin: 1rem;
        text-align: center;
    }

    #canvas-container {
        scale: 3;
        visibility: hidden;
    }
</style>