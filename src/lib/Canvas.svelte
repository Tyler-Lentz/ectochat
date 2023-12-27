<script lang="ts">
	import { onMount } from "svelte";
    import brushIcon from "$lib/icons/brush.svg";
    import eraserIcon from "$lib/icons/eraser.svg";
	import { openModal } from "svelte-modals";
	import PromptModal from "./PromptModal.svelte";
	import { writable } from "svelte/store";

    export let width: number = -1;
    export let height: number = -1;
    export let editable: boolean;
    export let scale: number = 1;
    export let data: number[] | undefined = undefined;

    let color: string;
    const colors = [
        "black",
        "red",
        "blue",
        "orange",
        "green",
        "violet"
    ];

    function getInverseColor(color: string) {
        switch (color) {
            case "red":
            case "orange":
            case "violet":
                return "var(--ctp-latte-blue)";
            default: 
                return "var(--ctp-latte-rosewater)";
        }
    }

    // Have to use the filter css attribute to change the color of an
    // svg icon easily, so this function just returns the right filter for the 
    // current color, from this website: https://angel-rs.github.io/css-color-filter-generator/
    function getCurrentFilter(col: string) {
        switch (col) {
            case "black": return "brightness(0) saturate(100%)";
            case "red": return "brightness(0) saturate(100%) invert(20%) sepia(70%) saturate(6966%) hue-rotate(2deg) brightness(96%) contrast(129%)"
            case "blue": return "brightness(0) saturate(100%) invert(8%) sepia(99%) saturate(7452%) hue-rotate(246deg) brightness(92%) contrast(144%)"
            case "orange": return "brightness(0) saturate(100%) invert(63%) sepia(46%) saturate(1578%) hue-rotate(359deg) brightness(101%) contrast(107%)"
            case "green": return "brightness(0) saturate(100%) invert(28%) sepia(93%) saturate(1296%) hue-rotate(93deg) brightness(93%) contrast(104%)"
            case "violet": return "brightness(0) saturate(100%) invert(23%) sepia(100%) saturate(6950%) hue-rotate(273deg) brightness(95%) contrast(128%)"
        }
    }

    let canvas: HTMLCanvasElement;
    let context: CanvasRenderingContext2D | null;
    let is_drawing: boolean = false;
    let has_moved: boolean = false;
    let prev_point: {x: number, y: number} = {x: 0, y: 0};
    let line_width: number = 2;
    $: {
        if (context != null) {
            context.lineWidth = line_width;
        }
    }

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

    export function getFormattedImageData() {
        return (getImageData()?.data || []).toString();
    }

    export function setImageData(data: ImageData) {
        context?.putImageData(data, 0, 0);
    }


    onMount(() => {
        context = canvas.getContext('2d');
        if (context != null) {
            context.lineWidth = 2;
            context.scale(scale, scale);
            updateColor('black');
        }

        // Set width/height to equal size of container if kept at the
        // default -1 values
        if (height == -1) {
            canvas.style.height = "100%";
            height = canvas.offsetHeight;
        }

        if (width == -1) {
            canvas.style.width = "100%";
            width = canvas.offsetWidth;
        }

        // put prepopulated data then put inside at start
        // data.length must be a non-zero multiple of 4 for it to be a valid
        // image data
        if (data != undefined && data.length > 0 && data.length % 4 == 0) {
            let imageData = new ImageData(
                new Uint8ClampedArray(data),
                width, 
                height,
            );
            setImageData(imageData);
        }
    });

    function onMouseDown(e: MouseEvent) {
        if (editable) {
            is_drawing = true;
            prev_point = {x: e.offsetX / scale, y: e.offsetY / scale};
            has_moved = false;
        }
    };

    function onMouseUp(e: MouseEvent) {
        if (editable) {
            is_drawing = false;

            if (!has_moved && context != null) {
                // Make sure single dot is displayed
                context.fillRect(e.offsetX / scale, e.offsetY / scale, context.lineWidth, context.lineWidth);
            }

            prev_point = {x: 0, y: 0};
        }
    }

    function onMouseMove(e: MouseEvent) {
        if (editable && is_drawing && context != null) {
            has_moved = true;

            context.beginPath();
            context.moveTo(prev_point.x, prev_point.y);
            context.lineTo(e.offsetX / scale, e.offsetY / scale);
            context.closePath();
            context.stroke();

            prev_point = {x: e.offsetX / scale, y: e.offsetY / scale};
        }
    }

    function handlePickColor(e: MouseEvent) {
        e.preventDefault();
        updateColor((e.target as HTMLButtonElement).value);
    }

    let erase_canvas = writable("");
    erase_canvas.subscribe((val) => {
        if (val === "Yes") {
            context?.clearRect(0, 0, canvas.width / scale, canvas.height / scale);
            erase_canvas.set("");
        }
    });

    function openEraseModal() {
        openModal(PromptModal, {
            message: "Erase Canvas?", 
            result: erase_canvas,
        });
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
        style:border-color={(editable) ? "" : "transparent"}
        >
    </canvas>
    <div id="palette">
        <div id="brush-type-controls">
            <img src={brushIcon} 
                alt={color}
                style:filter={getCurrentFilter(color)}
                width={12}
                height={8}
                >
            <img src={eraserIcon} 
                on:click={openEraseModal}
                id="eraser-img"
                alt={"Erase"}
                width={12}
                height={8}
                >
        </div>
        {#each colors as col}
            <button
                style:background={col} 
                on:click={handlePickColor}
                value={col}
                >
            </button>
        {/each}
        <div id="brush-width-controls">
            {#each [2, 4, 6] as width}
                <button
                    style:background={color}
                    style:border-color={(line_width == width) ? getInverseColor(color): ""}
                    style:width={width * 2 + "px"}
                    style:height={width * 2 + "px"}
                    on:click={() => {
                        line_width = width;
                    }}
                    >
                    </button>
            {/each}
        </div>
    </div> 
</div>

<style>

    #container {
        transition: scale 1s ease-in-out;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    }

    #brush-type-controls {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    #eraser-img {
        transition: translate 400ms ease-in;
    }

    #eraser-img:hover {
        translate: 0px -1px 0px;
    }

    canvas {
        border-radius: 16px;
        background-color: white;
        border: 1px solid var(--ctp-latte-overlay0);
    }

    #container:has(canvas[data-editable="true"]) > #palette {
        display: flex; /*only display if surrounding canvas is editable */
    }

    #palette {
        display: none; /* overridden in above rule */
        transition: opacity 0.5s ease-in;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;

        margin-left: 0.5em;

        border-radius: 4px;
        padding: 8px;
        background-color: var(--ctp-latte-mantle); 
    }

    #palette > button {
        width: 12px;
        height: 8px;
    }

    #brush-width-controls {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        margin-top: 2px;
    }

    #brush-width-controls > button {
        border-radius: 50%;
        padding: 0;
        margin-right: 2px;
        border: .5px solid transparent;
        transition: border-color 400ms ease-in-out;
    }

    #brush-width-controls > button:last-child {
        margin-right: 0;
    }

</style>