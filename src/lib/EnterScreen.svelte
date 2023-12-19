<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { profile } from '$lib/stores';
    import type { Profile } from '$lib/bindings/Profile';
    import Canvas from '$lib/Canvas.svelte';
    import { MAX_NAME_LEN, PROFILE_PIC_SIZE } from '$lib/contants';

    let entered_name: string = "";
    let display_name_error: boolean = false;
    let canvas: Canvas;
    let profile_pic_data: string = "";

    function createProfile() {
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

    function isNameValid(name: string) {
        return (name.length > 0 && name.length <= MAX_NAME_LEN);
    }

    function validateName() {
        if (isNameValid(entered_name)) {
            state++;
        } else {
            display_name_error = true;
        }
    }

    // Hide the name error if the users types in a valid name after
    $: {
        if (display_name_error && isNameValid(entered_name)) {
            display_name_error = false;
        }
    };

    function validatePicture() {
        profile_pic_data = (canvas.getImageData()?.data || []).toString();
        state++;
    }

</script>

<div class="container">
    <h1>ectochat</h1>

    {#if state == 0}
        <section id="name-section">
            <form on:submit={validateName}>
                <input 
                    type="text" 
                    maxlength={MAX_NAME_LEN}
                    placeholder="Enter your name" 
                    bind:value={entered_name}
                    />
                <p id="name-error" data-visible={display_name_error} >
                    Your name must be between 1 and {MAX_NAME_LEN} characters.
                </p>
                <input type="submit" value="Continue">
            </form>
        </section>
    {:else if state == 1}
        <section id="canvas-section">
            <form on:submit={validatePicture}>
                <div id="canvas-container">
                    <Canvas
                        bind:this={canvas}
                        width={PROFILE_PIC_SIZE}
                        height={PROFILE_PIC_SIZE}
                        editable={true}
                        />
                </div>
                <input type="submit" value="Continue" />
            </form>
        </section>
    {:else if state == 2}
    <div>
        <form on:submit={createProfile}>
            
            <input id="verify-profile-btn" type="submit" value="Start Chatting">
        </form>
    </div>
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

    #name-section {
        position: absolute;
        top: 40vh;
    }

    #name-error {
        /* https://css-irl.info/animating-underlines/ */
        color: var(--ctp-latte-red);
        background: 
            linear-gradient(to right, transparent, transparent),
            linear-gradient(to right, var(--ctp-latte-red), var(--ctp-latte-red));
        background-size: 100% 0.1em, 0 0.1em;
        background-position: 100% 100%, 0 100%;
        background-repeat: no-repeat;

        opacity: 0;
        transition: background-size 800ms, opacity 1200ms;
    }

    #name-error[data-visible="true"] {
        background-size: 0 0.1em, 100% 0.1em;
        opacity: 1;
        /* Overwrite the opacity transition so that when it goes from hidden -> shown it happens faster
           than when it goes from shown -> hidden */
        transition: background-size 800ms, opacity 400ms;
    }

    #canvas-section input[type="submit"] {
        position: absolute;
        top: 80vh;
    }

    #canvas-container {
        position: absolute;
        top: 40vh;
    }

    @media (min-width: 200px) {
        #canvas-container {
            scale: 1;
        }
    }
    @media (min-width: 400px) and (min-height: 400px) {
        #canvas-container {
            scale: 1.75;
        }
    }
    @media (min-width: 600px) and (min-height: 600px) {
        #canvas-container {
            scale: 2.5;
        }
    }
    @media (min-width: 800px) and (min-height: 800px) {
        #canvas-container {
            scale: 3.25;
        }
    }
    @media (min-width: 1000px) and (min-height: 800px) {
        #canvas-container {
            scale: 4;
        }
    }
    @media (min-width: 1200px) and (min-height: 800px) {
        #canvas-container {
            scale: 4.75;
        }
    }

    #verify-profile-btn {
        position: absolute;
        top: 40vh;
    }
</style>