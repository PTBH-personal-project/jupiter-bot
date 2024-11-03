<script lang="ts">
    export let text: string;
</script>

<div class="tooltip-wrapper">
    <slot />
    <span class="tooltip">{text}</span>
</div>

<style>
    .tooltip-wrapper {
        position: relative;
        display: inline-block;
    }

    .tooltip {
        visibility: hidden;
        position: absolute;
        background: rgba(0, 0, 0, 0.8);
        color: white;
        padding: 5px 10px;
        border-radius: 4px;
        font-size: 12px;
        white-space: nowrap;
        z-index: 1;
        bottom: 125%;
        left: 50%;
        transform: translateX(-50%);
        opacity: 0;
        transition: opacity 0.2s;
    }

    .tooltip::after {
        content: "";
        position: absolute;
        top: 100%;
        left: 50%;
        margin-left: -5px;
        border-width: 5px;
        border-style: solid;
        border-color: rgba(0, 0, 0, 0.8) transparent transparent transparent;
    }

    .tooltip-wrapper:hover .tooltip {
        visibility: visible;
        opacity: 1;
    }
</style>
