<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { TokenInfo } from "../types/tokens";

    let address = "";
    let tokenInfo: TokenInfo | null = null;
    let logoUri = "";
    let isLoading = false;

    async function fetchInfo() {
        isLoading = true;
        logoUri = "";
        try {
            tokenInfo = await invoke("get_token_info", { tokenAddress: address });
            if (tokenInfo && tokenInfo.uri) {
                try {
                    const response = await fetch(tokenInfo.uri);
                    const metadata = await response.json();
                    if (metadata.image) {
                        logoUri = metadata.image;
                    }
                } catch (error) {
                    console.error("Error fetching token metadata:", error);
                }
            }
        } finally {
            isLoading = false;
        }
    }
</script>

<main class="container">
    <h1>Welcome to Jupiter Bot</h1>

    <p>Paste the token address and get the overview of the token</p>

    <form class="row" on:submit|preventDefault={fetchInfo}>
        <input id="greet-input" placeholder="Enter an solana address..." bind:value={address} />
        <button type="submit">Fetch</button>
    </form>
    {#if isLoading}
        <div class="token-info">
            <h2 class="skeleton skeleton-text"></h2>
            <div class="info-grid">
                {#each Array(6) as _}
                    <div class="info-item">
                        <span class="label skeleton skeleton-text"></span>
                        <span class="value skeleton skeleton-text"></span>
                    </div>
                {/each}
            </div>
        </div>
    {:else if tokenInfo}
        <div class="token-info">
            <h2>Token Information</h2>
            <div class="info-grid">
                <div class="info-item">
                    <span class="label">Name:</span>
                    <div class="value-with-logo">
                        {#if logoUri}
                            <img src={logoUri} alt="Token Logo" class="token-logo" />
                        {/if}
                        <span class="value">{tokenInfo.name}</span>
                    </div>
                </div>
                <div class="info-item">
                    <span class="label">Symbol:</span>
                    <span class="value">{tokenInfo.symbol}</span>
                </div>
                <div class="info-item">
                    <span class="label">Decimals:</span>
                    <span class="value">{tokenInfo.decimals}</span>
                </div>
                <div class="info-item">
                    <span class="label">Address:</span>
                    <span class="value address">{tokenInfo.address}</span>
                </div>
                <div class="info-item">
                    <span class="label">Total Supply:</span>
                    <span class="value">
                        {new Intl.NumberFormat('en-US', {
                            maximumFractionDigits: 2,
                            minimumFractionDigits: 0,
                            useGrouping: true
                        }).format(tokenInfo.totalSupply / Math.pow(10, tokenInfo.decimals))}
                    </span>
                </div>
                <div class="info-item">
                    <span class="label">URI:</span>
                    <span class="value">{tokenInfo.uri}</span>
                </div>
            </div>
        </div>
    {/if}
</main>

<style>
    .logo.vite:hover {
        filter: drop-shadow(0 0 2em #747bff);
    }

    .logo.svelte-kit:hover {
        filter: drop-shadow(0 0 2em #ff3e00);
    }

    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: #0f0f0f;
        background-color: #f6f6f6;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    .logo {
        height: 6em;
        padding: 1.5em;
        will-change: filter;
        transition: 0.75s;
    }

    .logo.tauri:hover {
        filter: drop-shadow(0 0 2em #24c8db);
    }

    .row {
        display: flex;
        justify-content: center;
    }

    a {
        font-weight: 500;
        color: #646cff;
        text-decoration: inherit;
    }

    a:hover {
        color: #535bf2;
    }

    h1 {
        text-align: center;
    }

    input,
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #0f0f0f;
        background-color: #ffffff;
        transition:
            border-color 0.25s,
            width 0.3s ease;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }

    button {
        cursor: pointer;
    }

    button:hover {
        border-color: #396cd8;
    }
    button:active {
        border-color: #396cd8;
        background-color: #e8e8e8;
    }

    input,
    button {
        outline: none;
    }

    #greet-input {
        margin-right: 5px;
        width: 500px;
        font-family: monospace;
    }

    @media (prefers-color-scheme: dark) {
        :root {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }

        a:hover {
            color: #24c8db;
        }

        input,
        button {
            color: #ffffff;
            background-color: #0f0f0f98;
        }
        button:active {
            background-color: #0f0f0f69;
        }
    }

    .token-info {
        margin-top: 2rem;
        padding: 2rem;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 12px;
        backdrop-filter: blur(10px);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        max-width: 800px;
        margin-left: auto;
        margin-right: auto;
    }

    .token-info h2 {
        margin-bottom: 1.5rem;
        color: #396cd8;
        font-size: 1.5rem;
    }

    .info-grid {
        display: grid;
        gap: 1rem;
        max-width: 100%;
    }

    .info-item {
        display: grid;
        grid-template-columns: 150px 1fr; /* Fixed width for label, remaining space for value */
        padding: 1rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
        align-items: center;
    }

    .label {
        font-weight: 500;
        color: #666;
        text-align: left;
    }

    .value {
        font-family: monospace;
        word-break: break-all;
        text-align: left;
        padding-left: 1rem;
    }

    .address {
        color: #396cd8;
    }

    @media (prefers-color-scheme: dark) {
        .token-info {
            background: rgba(0, 0, 0, 0.2);
        }

        .info-item {
            background: rgba(255, 255, 255, 0.03);
        }

        .label {
            color: #999;
        }

        .address {
            color: #4a7be0;
        }
        .value-with-logo {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding-left: 1rem;
    }

    .token-logo {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        object-fit: cover;
    }
    }

    .skeleton {
        background: linear-gradient(
            90deg,
            rgba(255, 255, 255, 0.1),
            rgba(255, 255, 255, 0.2),
            rgba(255, 255, 255, 0.1)
        );
        background-size: 200% 100%;
        animation: loading 1.5s infinite;
        border-radius: 4px;
    }

    .skeleton-text {
        height: 1em;
        width: 100%;
    }

    @keyframes loading {
        0% {
            background-position: 200% 0;
        }
        100% {
            background-position: -200% 0;
        }
    }

    /* Dark mode adjustments */
    @media (prefers-color-scheme: dark) {
        .skeleton {
            background: linear-gradient(
                90deg,
                rgba(255, 255, 255, 0.05),
                rgba(255, 255, 255, 0.1),
                rgba(255, 255, 255, 0.05)
            );
            background-size: 200% 100%;
        }
    }
</style>
