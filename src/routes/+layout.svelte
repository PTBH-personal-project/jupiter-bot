<script lang="ts">
    import NavBar from "../components/NavBar.svelte";
    import { fly } from "svelte/transition";

    let isNavOpen = true;

    function handleNavToggle(event: CustomEvent) {
        isNavOpen = event.detail.isOpen;
    }
</script>

<NavBar {isNavOpen} on:navToggle={handleNavToggle} />

<main class:nav-open={isNavOpen} transition:fly={{ x: isNavOpen ? 250 : 0, duration: 300 }}>
    <slot />
</main>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        font-family: Arial, sans-serif;
    }

    main {
        padding: 20px;
        padding-left: 84px; /* Increased to account for collapsed navbar */
        transition: all 0.3s ease-in-out;
        min-height: 100vh;
    }

    main.nav-open {
        padding-left: 270px; /* Increased to account for expanded navbar */
    }
</style>
