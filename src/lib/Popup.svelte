<script lang="ts">
	import { onMount } from "svelte";

    export let message: string;
    export let display_time: number = 5000;
    export let slide_time: number = 500;
    
    let container: HTMLDivElement;

    onMount(() => {
        setTimeout(() => {
            container.style.translate = '0 -1lh'; 
            setTimeout(() => {
                container.style.translate = '0 0';
                setTimeout(() => {
                    container.parentElement?.removeChild(container);
                }, slide_time);
            }, display_time);
        }, 0);
    });
</script>

<div class="container" style="--slide-time: {slide_time}ms" bind:this={container}>
    {message}
</div>

<style>
    .container {
        background-color: var(--ctp-latte-crust);
        color: var(--ctp-latte-blue);
        width: 100vw;
        border-top: var(--ctp-latte-overlay2) 1px solid;
        font-size: 14pt;
        text-align: center;
        padding: 0.5em;
        position: absolute;
        bottom: -1lh;
        transition: translate var(--slide-time) linear;
    }
</style>