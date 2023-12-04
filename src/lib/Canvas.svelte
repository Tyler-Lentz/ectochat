<script lang="ts">
	import { onMount } from "svelte";

    export let width: number;
    export let height: number;
    export let editable: boolean;

    let color: string;
    const colors = [
        "black",
        "white",
        "red",
        "yellow",
        "blue",
        "orange",
        "green",
        "violet"
    ];

    let canvas: HTMLCanvasElement;
    let context: CanvasRenderingContext2D | null;
    let is_drawing: boolean = false;
    let has_moved: boolean = false;
    let prev_point: {x: number, y: number} = {x: 0, y: 0};

    function updateColor(new_color: string) {
        color = new_color;
        if (context != null) {
            context.strokeStyle = color;
            context.fillStyle = color;
        }
    }

    export function getImageData() {
        return context?.getImageData(0, 0, width, height);
    }

    export function setImageData(data: ImageData) {
        context?.putImageData(data, 0, 0);
    }


    onMount(() => {
        context = canvas.getContext('2d');
        if (context != null) {
            context.lineWidth = 2;
            updateColor('black');
        }
    });

    function onMouseDown(e: MouseEvent) {
        if (editable) {
            is_drawing = true;
            prev_point = {x: e.offsetX, y: e.offsetY};
            has_moved = false;
        }
    };

    function onMouseUp(e: MouseEvent) {
        if (editable) {
            is_drawing = false;

            if (!has_moved && context != null) {
                // Make sure single dot is displayed
                context.fillRect(e.offsetX, e.offsetY, context.lineWidth, context.lineWidth);
            }

            prev_point = {x: 0, y: 0};
        }
    }

    function onMouseMove(e: MouseEvent) {
        if (editable && is_drawing && context != null) {
            has_moved = true;

            context.beginPath();
            context.moveTo(prev_point.x, prev_point.y);
            context.lineTo(e.offsetX, e.offsetY);
            context.closePath();
            context.stroke();

            prev_point = {x: e.offsetX, y: e.offsetY};
        }
    }

    function handlePickColor(e: MouseEvent) {
        e.preventDefault();
        updateColor((e.target as HTMLButtonElement).value)
    }

</script>

<div id="container">
    <canvas 
        {width} 
        {height}
        bind:this={canvas}
        data-editable={editable}
        on:mousedown={onMouseDown}
        on:mouseup={onMouseUp}
        on:mouseleave={onMouseUp}
        on:mousemove={onMouseMove}
        >
    </canvas>
    <div id="palette">
        {#each colors as col}
            <button
                style:background={col} 
                style:outline={(color==col) ? '1px double var(--ctp-latte-overlay1)' : ''}
                on:click={handlePickColor}
                value={col}
                >
            </button>
        {/each}
    </div> 
</div>

<style>
    #container {
        scale: 1;
        transition: scale 0.5s ease-in-out;
    }

    #container:hover:has(canvas[data-editable="true"]) {
        scale: 3;
    }

    canvas {
        border-radius: 16px;
        background-color: white;
        border: 1px solid var(--ctp-latte-overlay1);
    }

    #container:hover:has(canvas[data-editable="true"]) > #palette {
        opacity: 1;
    }

    #container:has(canvas[data-editable="true"]) > #palette {
        display: flex; /*only display if surrounding canvas is editable */
    }

    #palette {
        display: none; /* overridden in above rule */
        opacity: 0;
        transition: opacity 0.5s ease-in;
        flex-direction: row;
        justify-content: space-between;
    }

    #palette > button {
        width: 8px;
        height: 8px;
        margin-bottom: 1em;
    }
</style>