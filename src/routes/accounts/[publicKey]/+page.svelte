<script lang="ts">
    import { page } from "$app/stores";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { Account } from "../../../types/accounts";
    import type { TokenAccount } from "../../../types/tokens";

    const publicKey = $page.params.publicKey;
    let account: Account | null = null;
    let balance: number | null = null;
    let isLoading = true;
    let isBalanceLoading = true;
    let error: string | null = null;
    let tokenAccounts: TokenAccount[] = [];
    let isTokenAccountsLoading = true;
    let tokenAccountsError: string | null = null;

    onMount(() => {
        loadAccountDetails().then(() => {
            fetchBalance();
            loadTokenAccounts();
        });
    });

    async function loadAccountDetails() {
        isLoading = true;
        error = null;
        try {
            account = await invoke("get_account_by_public_key", {
                publicKey: publicKey,
            });
        } catch (err) {
            console.error("Error loading account details:", err);
            error = err instanceof Error ? err.message : String(err);
        } finally {
            isLoading = false;
        }
    }

    async function fetchBalance() {
        if (!account) return;
        isBalanceLoading = true;
        try {
            const balanceInLamports = (await invoke("get_account_balance", {
                publicKey: account.public_key,
                tokenAddress: null,
            })) as number;
            balance = balanceInLamports / 1e9; // Convert lamports to SOL
        } catch (err) {
            console.error("Error fetching balance:", err);
            balance = null;
        } finally {
            isBalanceLoading = false;
        }
    }

    async function refreshBalance() {
        await fetchBalance();
    }

    async function loadTokenAccounts() {
        if (!account) return;
        isTokenAccountsLoading = true;
        tokenAccountsError = null;
        try {
            tokenAccounts = await invoke("get_all_token_account_for_pubkey", {
                pubkey: account.public_key,
            });
            console.log("tokenAccounts: ", tokenAccounts);
        } catch (err) {
            console.error("Error loading token accounts:", err);
            tokenAccountsError = err instanceof Error ? err.message : String(err);
        } finally {
            isTokenAccountsLoading = false;
        }
    }
</script>

<main class="container">
    <div class="header">
        <h1>Account Details</h1>
    </div>

    {#if isLoading}
        <div class="loading-state">
            <div class="skeleton skeleton-text" style="width: 60%;" />
            <div class="skeleton skeleton-text" style="width: 40%;" />
            <div class="skeleton skeleton-text" style="width: 80%;" />
        </div>
    {:else if error}
        <div class="error-state">
            <p>Error loading account details: {error}</p>
        </div>
    {:else if account}
        <div class="account-details">
            <div class="detail-group">
                <div class="detail-item">
                    <label>Name</label>
                    <div class="value-container">
                        <span class="value">{account.name}</span>
                    </div>
                </div>
                <div class="detail-item">
                    <label>Status</label>
                    <div class="value-container">
                        <span class="status-text {account.status.toLowerCase()}">
                            {account.status}
                        </span>
                    </div>
                </div>
                <div class="detail-item">
                    <label>Balance</label>
                    <div class="value-container">
                        {#if isBalanceLoading}
                            <span class="skeleton skeleton-text" style="width: 80px;"></span>
                        {:else if balance !== null}
                            <div class="balance-wrapper">
                                <span class="value">{balance.toFixed(4)} SOL</span>
                                <button
                                    class="refresh-button"
                                    on:click={refreshBalance}
                                    title="Refresh balance"
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="16"
                                        height="16"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    >
                                        <path
                                            d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.3"
                                        />
                                    </svg>
                                </button>
                            </div>
                        {:else}
                            <span class="error-text">Error loading balance</span>
                        {/if}
                    </div>
                </div>
            </div>

            <div class="detail-item">
                <label>Public Key</label>
                <div class="value-container">
                    <span class="address">{account.public_key}</span>
                </div>
            </div>

            <div class="detail-item">
                <label>Description</label>
                <div class="value-container">
                    {account.description || "No description"}
                </div>
            </div>
        </div>
    {:else}
        <div class="error-state">
            <p>Account not found</p>
        </div>
    {/if}

    {#if account}
        <div class="token-accounts-section">
            <h2>Token Accounts</h2>

            {#if isTokenAccountsLoading}
                <div class="loading-state">
                    <div class="skeleton skeleton-text" style="width: 60%;" />
                    <div class="skeleton skeleton-text" style="width: 40%;" />
                </div>
            {:else if tokenAccountsError}
                <div class="error-state">
                    <p>Error loading token accounts: {tokenAccountsError}</p>
                </div>
            {:else if tokenAccounts.length === 0}
                <div class="empty-state">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <circle cx="12" cy="12" r="10" />
                        <line x1="12" y1="8" x2="12" y2="12" />
                        <line x1="12" y1="16" x2="12.01" y2="16" />
                    </svg>
                    <p>No token accounts found for this wallet</p>
                </div>
            {:else}
                <div class="token-accounts-table-container">
                    <table class="token-accounts-table">
                        <thead>
                            <tr>
                                <th>Token Account</th>
                                <th>Mint</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each tokenAccounts as account}
                                <tr>
                                    <td class="address-cell">
                                        <span class="address">{account.pubkey}</span>
                                    </td>
                                    <td class="address-cell">
                                        <span class="address">{account.mint}</span>
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/if}
        </div>
    {/if}
</main>

<style>
    .container {
        padding: 2rem;
        max-width: 1200px;
        margin: 0 auto;
    }

    .header {
        margin-bottom: 2rem;
    }

    .account-details {
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
        padding: 0.75rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .detail-group {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
        margin-bottom: 0.75rem;
        align-items: flex-start;
    }

    .detail-item {
        display: flex;
        flex-direction: column;
        height: 42px;
        flex: 0 0 auto;
        margin-right: 1rem;
        justify-content: space-between;
    }

    .detail-item label {
        font-size: 0.875rem;
        color: #666;
        font-weight: 500;
        line-height: 1;
    }

    .value-container {
        height: 24px;
        display: flex;
        align-items: center;
    }

    .value,
    .status-text {
        font-size: 1rem;
        line-height: 1;
    }

    .balance-wrapper {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        height: 24px;
    }

    .address {
        font-family: monospace;
        word-break: break-all;
    }

    .status-text {
        font-weight: 500;
        font-size: 1rem;
        line-height: 1;
    }

    .status-text.enabled {
        color: rgb(34, 197, 94);
    }

    .status-text.disabled {
        color: rgb(239, 68, 68);
    }

    @media (prefers-color-scheme: dark) {
        .status-text.enabled {
            color: rgb(74, 222, 128);
        }

        .status-text.disabled {
            color: rgb(248, 113, 113);
        }
    }

    .loading-state {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        padding: 2rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 12px;
    }

    .error-state {
        text-align: center;
        padding: 3rem;
        color: #ef4444;
        background: rgba(239, 68, 68, 0.1);
        border-radius: 12px;
    }

    .skeleton {
        background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
        background-size: 200% 100%;
        animation: loading 1.5s infinite;
        border-radius: 4px;
    }

    .skeleton-text {
        height: 1em;
    }

    @keyframes loading {
        0% {
            background-position: 200% 0;
        }
        100% {
            background-position: -200% 0;
        }
    }

    @media (prefers-color-scheme: dark) {
        .detail-item label {
            color: #999;
        }

        .skeleton {
            background: linear-gradient(90deg, #222 25%, #333 50%, #222 75%);
            background-size: 200% 100%;
        }

        .account-details {
            background: rgba(255, 255, 255, 0.03);
        }
    }

    .refresh-button {
        background: none;
        border: none;
        padding: 2px;
        height: 20px;
        width: 20px;
        cursor: pointer;
        color: #666;
        border-radius: 50%;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease-in-out;
    }

    .refresh-button:hover {
        color: #396cd8;
        background-color: rgba(57, 108, 216, 0.1);
    }

    .refresh-button:active {
        transform: scale(0.95);
    }

    .error-text {
        color: #ef4444;
    }

    @media (prefers-color-scheme: dark) {
        .refresh-button {
            color: #999;
        }

        .refresh-button:hover {
            color: #4a7be0;
            background-color: rgba(74, 123, 224, 0.1);
        }

        .error-text {
            color: #f87171;
        }
    }

    .token-accounts-section {
        margin-top: 2rem;
    }

    .token-accounts-section h2 {
        font-size: 1.25rem;
        font-weight: 600;
        margin-bottom: 1rem;
    }

    .empty-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 3rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
        color: #666;
        gap: 1rem;
    }

    .empty-state svg {
        color: #666;
    }

    .empty-state p {
        font-size: 1rem;
        text-align: center;
    }

    @media (prefers-color-scheme: dark) {
        .empty-state {
            background: rgba(255, 255, 255, 0.03);
            color: #999;
        }

        .empty-state svg {
            color: #999;
        }
    }

    .token-accounts-table-container {
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
        overflow-x: auto;
    }

    .token-accounts-table {
        width: 100%;
        border-collapse: collapse;
        font-size: 0.875rem;
    }

    .token-accounts-table th {
        text-align: left;
        padding: 1rem;
        font-weight: 500;
        color: #666;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }

    .token-accounts-table td {
        padding: 0.75rem 1rem;
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    }

    .token-accounts-table tr:last-child td {
        border-bottom: none;
    }

    .address-cell {
        min-width: 300px;
        font-family: monospace;
    }

    @media (prefers-color-scheme: dark) {
        .token-accounts-table th {
            color: #999;
        }

        .token-accounts-table-container {
            background: rgba(255, 255, 255, 0.03);
        }

        .token-accounts-table td {
            border-bottom-color: rgba(255, 255, 255, 0.03);
        }
    }
</style>
