<script lang="ts">
    import NavBar from "../components/NavBar.svelte";
    import { fly } from "svelte/transition";

    let isNavOpen = false;

    function handleNavToggle(event: CustomEvent) {
        isNavOpen = event.detail.isOpen;
    }
</script>

<NavBar on:navToggle={handleNavToggle} />

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
        padding-left: 60px; /* Adjust this value based on your toggle button size */
        transition: margin-left 0.3s ease-in-out;
    }

    main.nav-open {
        margin-left: 250px; /* This should match the width of your navbar */
    }
</style>
