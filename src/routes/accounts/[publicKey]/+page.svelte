<script lang="ts">
    import { page } from "$app/stores";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { Account } from "../../../types/accounts";
    import type { TokenAccount, TokenInfo } from "../../../types/tokens";

    const publicKey = $page.params.publicKey;
    let account: Account | null = null;
    let balance: number | null = null;
    let isLoading = true;
    let isBalanceLoading = true;
    let error: string | null = null;
    let tokenAccounts: TokenAccount[] = [];
    let isTokenAccountsLoading = true;
    let tokenAccountsError: string | null = null;
    let tokenInfoMap: Map<string, TokenInfo | null> = new Map();
    let tokenInfoLoading: Map<string, boolean> = new Map();

    // Add these variables for the confirmation dialog
    let showDeleteConfirmation = false;
    let tokenAccountToDelete: string | null = null;

    // Add notification state variables
    let showNotification = false;
    let notificationMessage = "";
    let notificationType: "success" | "error" = "success";

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

    async function fetchTokenInfo(mintAddress: string) {
        if (tokenInfoLoading.get(mintAddress) || tokenInfoMap.has(mintAddress)) return;

        tokenInfoLoading.set(mintAddress, true);
        tokenInfoMap = tokenInfoMap; // Trigger reactivity

        try {
            const tokenInfo = (await invoke("get_token_info", {
                tokenAddress: mintAddress,
            })) as TokenInfo;

            // Fetch logo from URI if logoUri is empty
            if (!tokenInfo.logoUri && tokenInfo.uri) {
                try {
                    const response = await fetch(tokenInfo.uri);
                    const metadata = await response.json();
                    if (metadata.image) {
                        tokenInfo.logoUri = metadata.image;
                    }
                } catch (error) {
                    console.error("Error fetching token metadata:", error);
                }
            }

            tokenInfoMap.set(mintAddress, tokenInfo);
            tokenInfoMap = tokenInfoMap; // Trigger reactivity
        } catch (err) {
            console.error(`Error fetching token info for ${mintAddress}:`, err);
            tokenInfoMap.set(mintAddress, null);
            tokenInfoMap = tokenInfoMap; // Trigger reactivity
        } finally {
            tokenInfoLoading.set(mintAddress, false);
            tokenInfoLoading = tokenInfoLoading; // Trigger reactivity
        }
    }

    async function loadTokenAccounts() {
        if (!account) return;
        isTokenAccountsLoading = true;
        tokenAccountsError = null;
        try {
            tokenAccounts = await invoke("get_all_token_account_for_pubkey", {
                pubkey: account.public_key,
            });
            // Fetch token info for each mint address
            tokenAccounts.forEach((account) => {
                fetchTokenInfo(account.mint);
            });
        } catch (err) {
            console.error("Error loading token accounts:", err);
            tokenAccountsError = err instanceof Error ? err.message : String(err);
        } finally {
            isTokenAccountsLoading = false;
        }
    }

    async function refreshTokenInfo(mintAddress: string) {
        // Clear existing token info to force refresh
        tokenInfoMap.delete(mintAddress);
        tokenInfoMap = tokenInfoMap;
        // Fetch new token info
        await fetchTokenInfo(mintAddress);
    }

    // Update the delete function
    async function deleteTokenAccount(pubkey: string) {
        try {
            await invoke("delete_token_account", {
                owner: $page.params.publicKey,
                tokenAccountPubkey: pubkey,
            });
            // Remove the token account from the list
            tokenAccounts = tokenAccounts.filter((account) => account.pubkey !== pubkey);
            showDeleteConfirmation = false;
            tokenAccountToDelete = null;

            // Show success notification
            notificationMessage = "Token account deleted successfully";
            notificationType = "success";
            showNotification = true;

            // Hide notification after 3 seconds
            setTimeout(() => {
                showNotification = false;
            }, 3000);
        } catch (err) {
            console.error("Error deleting token account:", err);
            // Show error notification
            notificationMessage = "Failed to delete token account";
            notificationType = "error";
            showNotification = true;

            // Hide notification after 3 seconds
            setTimeout(() => {
                showNotification = false;
            }, 3000);
        }
    }

    // Add this function to handle delete button click
    function handleDeleteClick(pubkey: string) {
        tokenAccountToDelete = pubkey;
        showDeleteConfirmation = true;
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
                                <th>Token</th>
                                <th>Token Account</th>
                                <th>Mint</th>
                                <th>Amount</th>
                                <th>Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each tokenAccounts as account}
                                <tr>
                                    <td class="token-cell">
                                        {#if tokenInfoLoading.get(account.mint)}
                                            <div
                                                class="skeleton skeleton-text"
                                                style="width: 120px;"
                                            />
                                        {:else}
                                            <div class="tooltip-wrapper">
                                                <div
                                                    class="token-info clickable"
                                                    on:click={() => refreshTokenInfo(account.mint)}
                                                    role="button"
                                                    tabindex="0"
                                                    data-tooltip="Refresh"
                                                >
                                                    {#if tokenInfoMap.get(account.mint)?.logoUri}
                                                        <img
                                                            src={tokenInfoMap.get(account.mint)
                                                                ?.logoUri}
                                                            alt={tokenInfoMap.get(account.mint)
                                                                ?.symbol || "Token"}
                                                            class="token-logo"
                                                        />
                                                    {/if}
                                                    <span class="token-symbol">
                                                        {tokenInfoMap.get(account.mint)?.symbol ||
                                                            "N/A"}
                                                    </span>
                                                </div>
                                                <span class="tooltip">{@html "Refresh"}</span>
                                            </div>
                                        {/if}
                                    </td>
                                    <td class="address-cell">
                                        <span class="address">{account.pubkey}</span>
                                    </td>
                                    <td class="address-cell">
                                        <span class="address">{account.mint}</span>
                                    </td>
                                    <td class="amount-cell">
                                        <span class="amount">{account.amount}</span>
                                    </td>
                                    <td class="action-cell">
                                        <div class="tooltip-wrapper">
                                            <button
                                                class="delete-button"
                                                on:click={() => handleDeleteClick(account.pubkey)}
                                                role="button"
                                                tabindex="0"
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
                                                    <path d="M3 6h18" />
                                                    <path
                                                        d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"
                                                    />
                                                    <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
                                                </svg>
                                            </button>
                                            <span class="tooltip">{@html "Delete"}</span>
                                        </div>
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

<!-- Update the confirmation dialog -->
{#if showDeleteConfirmation}
    <div class="popup-overlay">
        <div class="popup-content confirm-dialog">
            <h2>Delete Token Account</h2>
            <p>Are you sure to delete this token account and redeem SOL?</p>
            <div class="button-group">
                <button
                    class="popup-button cancel-button"
                    on:click={() => {
                        showDeleteConfirmation = false;
                        tokenAccountToDelete = null;
                    }}
                >
                    Cancel
                </button>
                <button
                    class="popup-button delete-button-confirm"
                    on:click={() =>
                        tokenAccountToDelete && deleteTokenAccount(tokenAccountToDelete)}
                >
                    Delete
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- Add notification component at the end of the file, after the confirmation dialog -->
{#if showNotification}
    <div class="notification {notificationType}">
        {notificationMessage}
    </div>
{/if}

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
        text-align: center;
        padding: 1rem;
        font-weight: 500;
        color: #666;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }

    .token-accounts-table th:first-child {
        text-align: left;
    }

    .token-accounts-table th:nth-child(4) {
        text-align: left;
        padding-left: 1rem;
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

    .amount-cell {
        min-width: 120px;
        text-align: left;
        padding-left: 1rem;
        font-family: monospace;
    }

    .amount {
        font-size: 0.875rem;
        display: block;
        text-align: left;
        width: 100%;
    }

    .token-cell {
        min-width: 150px;
        padding-right: 1rem;
    }

    .token-info {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        flex: 1;
    }

    .token-logo {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        object-fit: cover;
    }

    .token-symbol {
        font-weight: 500;
        font-size: 0.875rem;
    }

    .token-cell-wrapper {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
    }

    .token-info.clickable {
        cursor: pointer;
        padding: 0.25rem;
        border-radius: 4px;
        transition: background-color 0.2s;
    }

    .token-info.clickable:hover {
        background: rgba(255, 255, 255, 0.05);
    }

    @media (prefers-color-scheme: dark) {
        .token-info.clickable:hover {
            background: rgba(255, 255, 255, 0.03);
        }
    }

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

    @media (prefers-color-scheme: dark) {
        .tooltip {
            background: rgba(0, 0, 0, 0.9);
        }

        .tooltip::after {
            border-color: rgba(0, 0, 0, 0.9) transparent transparent transparent;
        }
    }

    .action-cell {
        width: 48px;
        text-align: center;
        padding: 0 0.5rem;
    }

    .delete-button {
        background: none;
        border: none;
        padding: 0.25rem;
        cursor: pointer;
        color: #666;
        border-radius: 4px;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease-in-out;
    }

    .delete-button:hover {
        color: #ef4444;
        background: rgba(239, 68, 68, 0.1);
    }

    .delete-button:active {
        transform: scale(0.95);
    }

    @media (prefers-color-scheme: dark) {
        .delete-button {
            color: #999;
        }

        .delete-button:hover {
            color: #f87171;
            background: rgba(248, 113, 113, 0.1);
        }
    }

    /* Update the confirmation dialog styles */
    .popup-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .popup-content {
        background-color: #fff;
        padding: 20px;
        border-radius: 8px;
        width: 80%;
        max-width: 400px;
    }

    .popup-content h2 {
        margin: 0 0 1rem 0;
        font-size: 1.25rem;
    }

    .popup-content p {
        margin: 0 0 1.5rem 0;
        color: #666;
    }

    .button-group {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
    }

    .popup-button {
        padding: 12px 24px;
        font-size: 1.1em;
        font-weight: 500;
        border-radius: 8px;
        border: 1px solid transparent;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        min-width: 120px;
    }

    .delete-button-confirm {
        background-color: #dc3545;
        color: white;
    }

    .delete-button-confirm:hover {
        background-color: #c82333;
        transform: translateY(-1px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    }

    .delete-button-confirm:active {
        transform: translateY(0);
        background-color: #bd2130;
    }

    @media (prefers-color-scheme: dark) {
        .popup-content {
            background-color: #2f2f2f;
            color: #f6f6f6;
        }

        .popup-content p {
            color: #9ca3af;
        }

        .delete-button-confirm {
            background-color: #dc3545;
        }

        .delete-button-confirm:hover {
            background-color: #bd2130;
        }
    }

    /* Add notification styles */
    .notification {
        position: fixed;
        bottom: 20px;
        right: 20px;
        padding: 12px 24px;
        border-radius: 8px;
        color: white;
        font-weight: 500;
        z-index: 1000;
        animation: slideIn 0.3s ease-out;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .notification.success {
        background-color: #28a745;
    }

    .notification.error {
        background-color: #dc3545;
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
        .notification.success {
            background-color: #2ea043;
        }

        .notification.error {
            background-color: #da3633;
        }
    }
</style>
