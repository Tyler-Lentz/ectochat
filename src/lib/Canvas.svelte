<script lang="ts">
	import { onMount } from "svelte";

    export let color: string;
    export let width: number;
    export let height: number;
    export let editable: boolean;

    let canvas: HTMLCanvasElement;
    let context: CanvasRenderingContext2D | null;
    let is_drawing: boolean = false;
    let has_moved: boolean = false;
    let prev_point: {x: number, y: number} = {x: 0, y: 0};

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
            context.strokeStyle = color;
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

</script>

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

<style>
    canvas {
        border-radius: 16px;
        background-color: white;
        border: 1px solid var(--ctp-latte-overlay1);
        transition: scale 0.5s ease-in-out;
    }

    canvas:hover[data-editable="true"] {
        scale: 3;
    }
</style>