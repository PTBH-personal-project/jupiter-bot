<script lang="ts">
  import { fly } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
  
  let isOpen = false;
  const dispatch = createEventDispatcher();

  function toggleNav() {
    isOpen = !isOpen;
    dispatch('navToggle', { isOpen });
  }
</script>

<div class="nav-container">
  <button class="toggle-btn" on:click={toggleNav}>
    {isOpen ? '←' : '→'}
  </button>

  {#if isOpen}
    <nav transition:fly={{ x: -250, duration: 300 }}>
      <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/about">About</a></li>
        <li><a href="/accounts">Accounts</a></li>
        <li><a href="/contact">Contact</a></li>
      </ul>
    </nav>
  {/if}
</div>

<style>
  .nav-container {
    position: fixed;
    top: 0;
    left: 0;
    height: 100vh;
    z-index: 1000;
  }

  .toggle-btn {
    position: absolute;
    top: 10px;
    left: 10px;
    z-index: 1001;
    background: #333;
    color: white;
    border: none;
    padding: 10px;
    cursor: pointer;
  }

  nav {
    background-color: #333;
    width: 250px;
    height: 100%;
    padding: 20px;
    box-sizing: border-box;
  }

  ul {
    list-style-type: none;
    padding: 0;
    margin: 0;
  }

  li {
    margin: 20px 0;
  }

  a {
    color: white;
    text-decoration: none;
    font-weight: bold;
  }

  a:hover {
    color: #24c8db;
  }
</style>
