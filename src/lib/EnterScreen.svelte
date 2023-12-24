<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { profile } from '$lib/stores';
    import type { Profile } from '$lib/bindings/Profile';
    import Canvas from '$lib/Canvas.svelte';
    import { MAX_NAME_LEN, PROFILE_PIC_SIZE } from '$lib/contants';
	import { appWindow } from '@tauri-apps/api/window';
	import { createEventDispatcher, onMount } from 'svelte';

    let entered_name: string = "";
    let display_name_error: boolean = false;
    let canvas: Canvas;

    function createProfile() {
        let profile_pic_data = canvas.getFormattedImageData();
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

        state++; // go to loading screen
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

	let initialized = false;

	appWindow.listen("evt_start_chatting", () => {
		initialized = true;
	});

    const dispatch = createEventDispatcher();

    function leaveEnterScreen(e: SubmitEvent) {
        e.preventDefault();
        if (initialized) {
            dispatch('initialized', {});
        }
    }

    let loading_dots = "";

    onMount(() => {
        setInterval(() => {
            if (loading_dots.length == 3) {
                loading_dots= "";
            } else {
                loading_dots += ".";
            }
        }, 800);
    });
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
            <div id="canvas-container">
                <Canvas
                    bind:this={canvas}
                    width={PROFILE_PIC_SIZE}
                    height={PROFILE_PIC_SIZE}
                    editable={true}
                    />
            </div>
            <input type="button" value="Continue" on:click={createProfile}>
        </section>
    {:else if state == 2}
        <span 
            id="loader" 
            style={(initialized) ? "animation-iteration-count: 1; opacity: 0;" : ""} 
            />
        <form on:submit={leaveEnterScreen}>
            <input data-visible={initialized} id="start-chatting-btn" type="submit" value="Start Chatting" />
        </form>
        <span data-visible={!initialized} id="loader-text">Loading{loading_dots}</span>
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

    #canvas-section input[type="button"] {
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

    #loader-text, #start-chatting-btn {
        color: var(--ctp-latte-blue);
        position: absolute;
        top: 80vh;
    }

    #start-chatting-btn {
        transition: opacity 500ms;
    }

    #loader-text[data-visible="false"],
    #start-chatting-btn[data-visible="false"] {
        opacity: 0;
    }

    #loader-text[data-visible="true"],
    #start-chatting-btn[data-visible="true"] {
        opacity: 1;
    }

    /* https://cssloaders.github.io/ */
    #loader {
        color: var(--ctp-latte-blue);
        top: 20vh;

        transition: opacity 2s;

        font-size: 45px;
        text-indent: -9999em;
        overflow: hidden;
        width: 1em;
        height: 1em;
        border-radius: 50%;
        position: relative;
        transform: translateZ(0);
        animation: mltShdSpin 1.7s infinite ease, round 1.7s infinite ease;
    }

    @keyframes mltShdSpin {
        0% {
            box-shadow: 0 -0.83em 0 -0.4em,
                0 -0.83em 0 -0.42em, 0 -0.83em 0 -0.44em,
                0 -0.83em 0 -0.46em, 0 -0.83em 0 -0.477em;
        }
        5%,
        95% {
            box-shadow: 0 -0.83em 0 -0.4em, 
                0 -0.83em 0 -0.42em, 0 -0.83em 0 -0.44em, 
                0 -0.83em 0 -0.46em, 0 -0.83em 0 -0.477em;
        }
        10%,
        59% {
            box-shadow: 0 -0.83em 0 -0.4em, 
                -0.087em -0.825em 0 -0.42em, -0.173em -0.812em 0 -0.44em, 
                -0.256em -0.789em 0 -0.46em, -0.297em -0.775em 0 -0.477em;
        }
        20% {
            box-shadow: 0 -0.83em 0 -0.4em, -0.338em -0.758em 0 -0.42em,
                -0.555em -0.617em 0 -0.44em, -0.671em -0.488em 0 -0.46em, 
                -0.749em -0.34em 0 -0.477em;
        }
        38% {
            box-shadow: 0 -0.83em 0 -0.4em, -0.377em -0.74em 0 -0.42em,
                -0.645em -0.522em 0 -0.44em, -0.775em -0.297em 0 -0.46em, 
                -0.82em -0.09em 0 -0.477em;
        }
        100% {
            box-shadow: 0 -0.83em 0 -0.4em, 0 -0.83em 0 -0.42em, 
                0 -0.83em 0 -0.44em, 0 -0.83em 0 -0.46em, 0 -0.83em 0 -0.477em;
        }
    }

    @keyframes round {
        0% { transform: rotate(0deg) }
        100% { transform: rotate(360deg) }
    }

</style>