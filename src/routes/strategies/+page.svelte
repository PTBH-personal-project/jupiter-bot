<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { Strategy } from "../../types/strategies";
    import type { Account } from "../../types/accounts";
    import Tooltip from "../../components/Tooltip.svelte";
    import type { TokenInfo } from "../../types/tokens";
    import { removeNullChars } from "$lib/utils/helpers";
    let showAddDialog = false;
    let strategies: Strategy[] = [];
    let isLoading = false;
    let error: string | null = null;

    // Form fields
    let strategyType: string = "Buy";
    let intervalTime: number = 3600; // 1 hour in seconds
    let tokenAddress: string = "";
    let price: number = 0;
    let amount: number = 0;
    let prioritizationFee: number = 5000;
    let slippage: number = 100; // 1%

    let notification = {
        show: false,
        message: "",
        isError: false,
    };

    let showDeleteConfirmDialog = false;
    let strategyToDelete: number | null = null;

    // Add accounts state
    let accounts: Account[] = [];
    let selectedAccountId: string = "";

    // Add tokens state
    let tokens: TokenInfo[] = [];

    onMount(async () => {
        await Promise.all([loadStrategies(), loadAccounts(), loadTokens()]);
    });

    async function loadStrategies() {
        try {
            strategies = await invoke("get_all_strategies");
        } catch (err) {
            console.error("Error loading strategies:", err);
            showNotification("Failed to load strategies: " + err, true);
        }
    }

    async function loadAccounts() {
        try {
            accounts = await invoke("get_accounts");
        } catch (err) {
            console.error("Error loading accounts:", err);
            showNotification("Failed to load accounts: " + err, true);
        }
    }

    async function loadTokens() {
        try {
            tokens = await invoke("get_all_tokens");
            console.log("Tokens loaded:", tokens);
        } catch (err) {
            console.error("Error loading tokens:", err);
            showNotification("Failed to load tokens: " + err, true);
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

    async function handleSubmit() {
        isLoading = true;
        error = null;

        try {
            // Find selected account
            const selectedAccount = accounts.find((acc) => acc.id.toString() === selectedAccountId);
            if (!selectedAccount) {
                throw new Error("Please select an account");
            }

            await invoke("add_strategy", {
                strategyType,
                intervalTime,
                accountPrivateKey: selectedAccount.private_key,
                tokenAddress,
                price,
                amount,
                prioritizationFee,
                slippage,
            });

            await loadStrategies();
            showNotification("Strategy added successfully!");
            closeDialog();
        } catch (err) {
            console.error("Error adding strategy:", err);
            error = err instanceof Error ? err.message : String(err);
            showNotification("Failed to add strategy: " + err, true);
        } finally {
            isLoading = false;
        }
    }

    function closeDialog() {
        showAddDialog = false;
        resetForm();
    }

    function resetForm() {
        strategyType = "Buy";
        intervalTime = 3600;
        selectedAccountId = "";
        tokenAddress = "";
        price = 0;
        amount = 0;
        prioritizationFee = 5000;
        slippage = 100;
        error = null;
    }

    async function deleteStrategy(id: number) {
        try {
            await invoke("delete_strategy", { id });
            await loadStrategies();
            showNotification("Strategy deleted successfully!");
        } catch (err) {
            console.error("Error deleting strategy:", err);
            showNotification("Failed to delete strategy: " + err, true);
        }
    }

    function showDeleteConfirm(id: number) {
        strategyToDelete = id;
        showDeleteConfirmDialog = true;
    }

    function closeDeleteConfirm() {
        showDeleteConfirmDialog = false;
        strategyToDelete = null;
    }

    async function confirmDelete() {
        if (strategyToDelete === null) return;

        await deleteStrategy(strategyToDelete);
        closeDeleteConfirm();
    }

    function shortenAddress(address: string): string {
        if (!address) return "";
        return `${address.slice(0, 4)}...${address.slice(-4)}`;
    }

    // Add this to help with debugging
    $: console.log("Tokens loaded:", tokens);
</script>

<main class="container">
    <div class="header">
        <h1>Strategies</h1>
    </div>

    <div class="actions">
        <button class="add-button" on:click={() => (showAddDialog = true)}> + Add Strategy </button>
    </div>

    <div class="strategies-container">
        {#if strategies.length === 0}
            <div class="empty-state">
                No strategies added yet. Click "Add Strategy" to create one.
            </div>
        {:else}
            <table class="strategies-table">
                <thead>
                    <tr>
                        <th>Type</th>
                        <th>Token</th>
                        <th>Price</th>
                        <th>Amount</th>
                        <th>Interval</th>
                        <th>Status</th>
                        <th>Next Execution</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each strategies as strategy}
                        <tr>
                            <td>{strategy.strategyType}</td>
                            <td class="address-cell">{strategy.tokenAddress}</td>
                            <td>{strategy.price}</td>
                            <td>{strategy.amount}</td>
                            <td>{strategy.intervalTime}s</td>
                            <td>
                                <span class="status-badge status-{strategy.status.toLowerCase()}">
                                    {strategy.status}
                                </span>
                            </td>
                            <td>
                                {new Date(strategy.nextTimeExecute * 1000).toLocaleString()}
                            </td>
                            <td>
                                <div class="action-buttons">
                                    <Tooltip text="Delete strategy">
                                        <button
                                            class="icon-button delete-button"
                                            on:click={() => showDeleteConfirm(strategy.id)}
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
                                                    d="M3 6h18M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"
                                                />
                                            </svg>
                                        </button>
                                    </Tooltip>
                                </div>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
    </div>

    {#if showAddDialog}
        <div class="dialog-overlay" on:click|self={closeDialog}>
            <div class="dialog">
                <div class="dialog-header">
                    <h2>Add Strategy</h2>
                    <button class="close-button" on:click={closeDialog}>×</button>
                </div>

                <form class="dialog-content" on:submit|preventDefault={handleSubmit}>
                    <div class="form-group">
                        <label for="strategy-type">Strategy Type</label>
                        <select
                            id="strategy-type"
                            class="select-input"
                            bind:value={strategyType}
                            required
                        >
                            <option value="Buy">Buy</option>
                            <option value="Sell">Sell</option>
                        </select>
                    </div>

                    <div class="form-group">
                        <label for="account">Account</label>
                        <select
                            id="account"
                            class="select-input"
                            bind:value={selectedAccountId}
                            required
                        >
                            <option value="">Select an account</option>
                            {#each accounts as account}
                                <option value={account.id}>
                                    {account.name} ({shortenAddress(account.public_key)})
                                </option>
                            {/each}
                        </select>
                    </div>

                    <div class="form-group">
                        <label for="interval-time">Interval Time (seconds)</label>
                        <input
                            type="number"
                            id="interval-time"
                            class="number-input"
                            bind:value={intervalTime}
                            min="1"
                        />
                    </div>

                    <div class="form-group">
                        <label for="token-address">Token</label>
                        <select
                            id="token-address"
                            class="select-input"
                            bind:value={tokenAddress}
                            required
                        >
                            <option value="">Select a token</option>
                            {#each tokens as token}
                                <option value={token.address} class="token-option">
                                    <!-- {token.name.trim() || 'Unknown'} ({token.symbol.trim() || '???'}) - {shortenAddress(token.address)} -->
                                    {removeNullChars(token.name)} ({shortenAddress(token.address)})
                                </option>
                            {/each}
                        </select>
                    </div>

                    <div class="form-group">
                        <label for="price">Price</label>
                        <input
                            type="number"
                            id="price"
                            class="number-input"
                            bind:value={price}
                            min="0"
                            step="0.000001"
                        />
                    </div>

                    <div class="form-group">
                        <label for="amount">Amount</label>
                        <input
                            type="number"
                            id="amount"
                            class="number-input"
                            bind:value={amount}
                            min="0"
                        />
                    </div>

                    <div class="form-group">
                        <label for="prioritization-fee">Prioritization Fee</label>
                        <input
                            type="number"
                            id="prioritization-fee"
                            class="number-input"
                            bind:value={prioritizationFee}
                            min="0"
                        />
                    </div>

                    <div class="form-group">
                        <label for="slippage">Slippage (%)</label>
                        <input
                            type="number"
                            id="slippage"
                            class="number-input"
                            bind:value={slippage}
                            min="0"
                            max="100"
                            step="0.1"
                        />
                    </div>

                    {#if error}
                        <div class="error-message">
                            <p>{error}</p>
                        </div>
                    {/if}

                    <div class="dialog-actions">
                        <button type="button" class="cancel-button" on:click={closeDialog}>
                            Cancel
                        </button>
                        <button type="submit" class="submit-button" disabled={isLoading}>
                            {#if isLoading}
                                <span class="loader" />
                            {:else}
                                Add Strategy
                            {/if}
                        </button>
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

    {#if showDeleteConfirmDialog}
        <div class="dialog-overlay" on:click|self={closeDeleteConfirm}>
            <div class="dialog">
                <div class="dialog-header">
                    <h2>Delete Strategy</h2>
                    <button class="close-button" on:click={closeDeleteConfirm}>×</button>
                </div>
                <div class="dialog-content">
                    <p>Are you sure you want to delete this strategy?</p>
                    <div class="dialog-actions">
                        <button type="button" class="cancel-button" on:click={closeDeleteConfirm}>
                            Cancel
                        </button>
                        <button type="button" class="delete-button" on:click={confirmDelete}>
                            Delete
                        </button>
                    </div>
                </div>
            </div>
        </div>
    {/if}
</main>

<style>
    /* Reuse existing styles from tokens page */
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

    /* Add new form-specific styles */
    .select-input,
    .number-input,
    .text-input {
        width: 100%;
        padding: 0.8rem;
        border: 1px solid #ddd;
        border-radius: 8px;
        font-size: 1rem;
        margin-bottom: 1rem;
        box-sizing: border-box;
    }

    .form-group {
        margin-bottom: 1.5rem;
    }

    .form-group label {
        display: block;
        margin-bottom: 0.5rem;
        font-weight: 500;
    }

    /* Reuse other styles from tokens page */
    /* ... (copy all other relevant styles from the tokens page) ... */

    /* Add these styles if not already present */
    .dialog-overlay {
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

    .dialog {
        background-color: white;
        border-radius: 8px;
        padding: 1.5rem;
        width: 90%;
        max-width: 500px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    }

    .dialog-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .dialog-content {
        margin-bottom: 1rem;
    }

    .dialog-actions {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
    }

    .delete-button {
        background-color: #dc2626;
        color: white;
    }

    .delete-button:hover {
        background-color: #b91c1c;
    }

    @media (prefers-color-scheme: dark) {
        .dialog {
            background-color: #1f2937;
            color: #f3f4f6;
        }

        .delete-button {
            background-color: #ef4444;
        }

        .delete-button:hover {
            background-color: #dc2626;
        }
    }

    /* Replace the primary-button style with this */
    .add-button {
        padding: 12px 24px;
        font-size: 1.1em;
        font-weight: 500;
        border-radius: 8px;
        border: 1px solid transparent;
        background-color: #396cd8;
        color: white;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        margin-bottom: 20px;
    }

    .add-button:hover {
        background-color: #2857b8;
        transform: translateY(-1px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    }

    .add-button:active {
        transform: translateY(0);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    @media (prefers-color-scheme: dark) {
        .add-button {
            background-color: #4a7be0;
        }

        .add-button:hover {
            background-color: #3967c4;
        }
    }

    /* Add these styles for the select input if not already present */
    .select-input {
        width: 100%;
        padding: 12px;
        border: 1px solid #ddd;
        border-radius: 8px;
        font-size: 1rem;
        background-color: white;
        cursor: pointer;
        appearance: none; /* Removes default browser styling */
        background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23666' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
        background-repeat: no-repeat;
        background-position: right 12px center;
        padding-right: 36px;
    }

    .select-input:focus {
        outline: none;
        border-color: #396cd8;
        box-shadow: 0 0 0 2px rgba(57, 108, 216, 0.1);
    }

    @media (prefers-color-scheme: dark) {
        .select-input {
            background-color: #374151;
            border-color: #4b5563;
            color: #f3f4f6;
            background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23999' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
        }

        .select-input:focus {
            border-color: #4a7be0;
            box-shadow: 0 0 0 2px rgba(74, 123, 224, 0.1);
        }

        .select-input option {
            background-color: #374151;
            color: #f3f4f6;
        }
    }

    /* Add this to ensure option text doesn't get cut off */
    .select-input option {
        padding: 8px 12px;
        min-height: 24px;
        white-space: normal; /* Allow text to wrap */
        word-wrap: break-word;
        line-height: 1.4;
        width: 100%;
        box-sizing: border-box;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .select-input {
            background-color: #374151;
            border-color: #4b5563;
            color: #f3f4f6;
        }

        .select-input option {
            background-color: #374151;
            color: #f3f4f6;
            padding: 8px 12px;
        }
    }
</style>
