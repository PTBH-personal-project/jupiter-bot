<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { TokenInfo } from "../../types/tokens";

    let showImportDialog = false;
    let address = "";
    let tokenInfo: TokenInfo | null = null;
    let logoUri = "";
    let isLoading = false;
    let tokenPrice: number | null = null;
    let isPriceLoading = false;
    let error: string | null = null;
    let importLoading = false;
    let notification = {
        show: false,
        message: "",
        isError: false,
    };

    async function fetchPrice() {
        if (!tokenInfo) return;
        isPriceLoading = true;
        try {
            tokenPrice = await invoke("get_token_price", {
                tokenAddress: tokenInfo.address,
                tokenDecimals: tokenInfo.decimals,
            });
        } catch (error) {
            console.error("Error fetching token price:", error);
        } finally {
            isPriceLoading = false;
        }
    }

    async function fetchInfo() {
        isLoading = true;
        logoUri = "";
        tokenPrice = null;
        error = null;
        try {
            tokenInfo = await invoke("get_token_info", { tokenAddress: address });
            if (tokenInfo) {
                fetchPrice();
                if (tokenInfo.uri) {
                    try {
                        const response = await fetch(tokenInfo.uri);
                        const metadata = await response.json();
                        if (metadata.image) {
                            logoUri = metadata.image;
                        }
                    } catch (error) {
                        console.error("Error fetching token metadata:", error);
                    }
                } else if (tokenInfo.logoUri) {
                    logoUri = tokenInfo.logoUri;
                }
            }
        } catch (err) {
            console.error("Error fetching token info:", err);
            error = err instanceof Error ? err.message : String(err);
            tokenInfo = null;
        } finally {
            isLoading = false;
        }
    }

    async function importToken() {
        if (!tokenInfo) return;

        importLoading = true;
        try {
            await invoke("import_token", {
                address: tokenInfo.address,
                symbol: tokenInfo.symbol,
                decimals: tokenInfo.decimals,
                name: tokenInfo.name,
                logoUri: logoUri,
                uri: tokenInfo.uri,
            });
            showNotification("Token imported successfully!");
            closeDialog();
        } catch (err) {
            console.error("Error importing token:", err);
            showNotification("Failed to import token: " + err, true);
        } finally {
            importLoading = false;
        }
    }

    function showNotification(message: string, isError = false) {
        notification = {
            show: true,
            message,
            isError,
        };
        setTimeout(() => {
            notification = {
                show: false,
                message: "",
                isError: false,
            };
        }, 3000);
    }

    function closeDialog() {
        showImportDialog = false;
        address = "";
        error = null;
        tokenInfo = null;
        tokenPrice = null;
        logoUri = "";
    }
</script>

<main class="container">
    <div class="header">
        <h1>Tokens</h1>
    </div>

    <div class="actions">
        <button class="primary-button" on:click={() => (showImportDialog = true)}>
            + Import Token
        </button>
    </div>

    {#if showImportDialog}
        <div class="dialog-overlay" on:click|self={closeDialog}>
            <div class="dialog">
                <div class="dialog-header">
                    <h2>Import Token</h2>
                    <button class="close-button" on:click={closeDialog}>×</button>
                </div>

                <form class="dialog-content" on:submit|preventDefault={fetchInfo}>
                    <input
                        type="text"
                        placeholder="Enter Solana token address..."
                        bind:value={address}
                        class="token-input"
                    />

                    {#if error}
                        <div class="error-message">
                            <p>{error}</p>
                        </div>
                    {:else if isLoading}
                        <div class="token-preview">
                            <div class="preview-grid">
                                {#each Array(7) as _}
                                    <div class="preview-item">
                                        <span class="label skeleton skeleton-text"></span>
                                        <span class="value skeleton skeleton-text"></span>
                                    </div>
                                {/each}
                            </div>
                        </div>
                    {:else if tokenInfo}
                        <div class="token-preview">
                            <div class="preview-grid">
                                <div class="preview-item">
                                    <span class="label">Name:</span>
                                    <div class="value-with-logo">
                                        <span class="value">{tokenInfo.name}</span>
                                        {#if logoUri}
                                            <img
                                                src={logoUri}
                                                alt="Token Logo"
                                                class="token-logo"
                                            />
                                        {/if}
                                    </div>
                                </div>
                                <div class="preview-item">
                                    <span class="label">Symbol:</span>
                                    <span class="value">{tokenInfo.symbol}</span>
                                </div>
                                <div class="preview-item">
                                    <span class="label">Decimals:</span>
                                    <span class="value">{tokenInfo.decimals}</span>
                                </div>
                                <div class="preview-item">
                                    <span class="label">Address:</span>
                                    <span class="value address">{tokenInfo.address}</span>
                                </div>
                                <div class="preview-item">
                                    <span class="label">Total Supply:</span>
                                    <span class="value">
                                        {new Intl.NumberFormat("en-US", {
                                            maximumFractionDigits: 2,
                                            minimumFractionDigits: 0,
                                            useGrouping: true,
                                        }).format(
                                            tokenInfo.totalSupply / Math.pow(10, tokenInfo.decimals)
                                        )}
                                    </span>
                                </div>
                                <div class="preview-item">
                                    <span class="label">URI:</span>
                                    <span class="value">{tokenInfo.uri}</span>
                                </div>
                                <div class="preview-item">
                                    <span class="label">Price (USDT):</span>
                                    <span class="value">
                                        {#if isPriceLoading}
                                            <span class="skeleton skeleton-text"></span>
                                        {:else if tokenPrice !== null}
                                            {(tokenPrice / Math.pow(10, 6)).toFixed(6)}
                                        {:else}
                                            Token not supported in Jupiter
                                        {/if}
                                    </span>
                                </div>
                            </div>
                        </div>
                    {/if}

                    <div class="dialog-actions">
                        <button type="button" class="cancel-button" on:click={closeDialog}>
                            Cancel
                        </button>
                        {#if !tokenInfo}
                            <button type="submit" class="submit-button" disabled={!address}>
                                {#if isLoading}
                                    <span class="loader"></span>
                                {:else}
                                    Search
                                {/if}
                            </button>
                        {:else}
                            <button
                                type="button"
                                class="submit-button"
                                on:click={importToken}
                                disabled={importLoading}
                            >
                                {#if importLoading}
                                    <span class="loader"></span>
                                {:else}
                                    Import
                                {/if}
                            </button>
                        {/if}
                    </div>
                </form>
            </div>
        </div>
    {/if}

    {#if notification.show}
        <div class="notification {notification.isError ? 'error' : 'success'}">
            {notification.message}
        </div>
    {/if}
</main>

<style>
    .container {
        padding: 2rem;
    }

    .header {
        margin-bottom: 1.5rem;
    }

    h1 {
        margin: 0;
        font-size: 2rem;
        font-weight: 600;
    }

    .actions {
        margin-bottom: 2rem;
    }

    .primary-button {
        background-color: #396cd8;
        color: white;
        border: none;
        padding: 0.8rem 1.5rem;
        border-radius: 8px;
        font-size: 1.1rem;
        font-weight: 500;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.7rem;
        transition: background-color 0.2s;
    }

    .primary-button:hover {
        background-color: #2857b8;
    }

    .error-message {
        margin-top: 1rem;
        padding: 0.75rem;
        background-color: #fee2e2;
        border: 1px solid #fecaca;
        border-radius: 8px;
        color: #dc2626;
    }

    .notification {
        position: fixed;
        top: 1rem;
        right: 1rem;
        padding: 1rem;
        border-radius: 8px;
        animation: slideIn 0.3s ease-out;
    }

    .notification.success {
        background-color: #d1fae5;
        border: 1px solid #a7f3d0;
        color: #047857;
    }

    .notification.error {
        background-color: #fee2e2;
        border: 1px solid #fecaca;
        color: #dc2626;
    }

    .token-preview {
        margin-top: 2rem;
        padding: 1rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
    }

    .token-info {
        display: flex;
        gap: 1rem;
        align-items: flex-start;
    }

    .token-logo {
        width: 64px;
        height: 64px;
        border-radius: 50%;
        object-fit: cover;
    }

    .token-details {
        flex: 1;
    }

    .token-details p {
        margin: 0.5rem 0;
    }

    .address {
        word-break: break-all;
        font-family: monospace;
        font-size: 0.9em;
    }

    .loader {
        display: inline-block;
        width: 20px;
        height: 20px;
        border: 3px solid rgba(255, 255, 255, 0.3);
        border-radius: 50%;
        border-top-color: white;
        animation: spin 1s ease-in-out infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    @keyframes slideIn {
        from {
            transform: translateX(100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }

    @media (prefers-color-scheme: dark) {
        .import-form {
            background: rgba(0, 0, 0, 0.2);
        }

        input {
            background: #1a1a1a;
            color: white;
            border-color: #333;
        }

        .token-preview {
            background: rgba(0, 0, 0, 0.3);
        }
    }

    .dialog-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .dialog {
        background: white;
        border-radius: 12px;
        width: 90%;
        max-width: 800px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        animation: slideIn 0.2s ease-out;
    }

    .dialog-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem;
        border-bottom: 1px solid #eee;
    }

    .dialog-header h2 {
        margin: 0;
        font-size: 1.5rem;
        color: #396cd8;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        color: #666;
        padding: 0.5rem;
    }

    .dialog-content {
        padding: 2rem;
        box-sizing: border-box;
    }

    .token-input {
        width: 100%;
        padding: 0.8rem;
        border: 1px solid #ddd;
        border-radius: 8px;
        font-size: 1rem;
        margin-bottom: 1rem;
        box-sizing: border-box;
    }

    .dialog-actions {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
        margin-top: 1.5rem;
    }

    .cancel-button,
    .submit-button {
        padding: 0.6rem 1.2rem;
        border-radius: 6px;
        font-size: 1rem;
        cursor: pointer;
        border: none;
    }

    .cancel-button {
        background-color: #e5e7eb;
        color: #374151;
    }

    .submit-button {
        background-color: #396cd8;
        color: white;
    }

    .submit-button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }

    @media (prefers-color-scheme: dark) {
        .dialog {
            background: #1a1a1a;
            color: white;
        }

        .dialog-header {
            border-bottom-color: #333;
        }

        .close-button {
            color: #999;
        }

        .token-input {
            background: #0f0f0f;
            border-color: #333;
            color: white;
        }

        .cancel-button {
            background-color: #374151;
            color: #e5e7eb;
        }
    }

    @keyframes slideIn {
        from {
            transform: translateY(-10px);
            opacity: 0;
        }
        to {
            transform: translateY(0);
            opacity: 1;
        }
    }

    .preview-grid {
        display: grid;
        gap: 1rem;
    }

    .preview-item {
        display: grid;
        grid-template-columns: 150px 1fr;
        padding: 0.75rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 6px;
        align-items: center;
    }

    .label {
        font-weight: 500;
        color: #666;
    }

    .value {
        font-family: monospace;
        word-break: break-all;
    }

    .value-with-logo {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .token-logo {
        width: 24px;
        height: 24px;
        border-radius: 50%;
    }

    .token-preview {
        margin: 1rem 0;
        padding: 1rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
    }

    @media (prefers-color-scheme: dark) {
        .preview-item {
            background: rgba(255, 255, 255, 0.03);
        }

        .label {
            color: #999;
        }
    }
</style>
