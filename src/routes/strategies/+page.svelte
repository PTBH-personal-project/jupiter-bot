<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { Strategy, StrategyWithFullInformation } from "../../types/strategies";
    import type { Account } from "../../types/accounts";
    import Tooltip from "../../components/Tooltip.svelte";
    import type { TokenInfo } from "../../types/tokens";
    import { removeNullChars } from "$lib/utils/helpers";
    let showAddDialog = false;
    let strategies: StrategyWithFullInformation[] = [];
    let isLoading = false;
    let error: string | null = null;

    // Form fields
    let strategyType: string = "Buy";
    let intervalTime: number = 20; // 20 seconds
    let tokenAddress: string = "";
    let price: number = 0;
    let amount: number = 0;
    let prioritizationFee: number = 5000;
    let slippage: number = 5; // 1%

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

    let isTokenDropdownOpen = false;
    let selectedToken: TokenInfo | null = null;

    let selectedTokenPrice: number | null = null;

    // Add new state variable for account balance
    let selectedAccountBalance: number | null = null;

    async function fetchTokenPrice(tokenAddress: string, tokenDecimals: number) {
        try {
            const onchainPrice = await invoke("get_token_price_in_sol", {
                tokenAddress,
                tokenDecimals,
            });
            selectedTokenPrice = Number(onchainPrice) / Math.pow(10, 9); // Assuming price is in lamports
            price = selectedTokenPrice;
        } catch (err) {
            console.error("Error fetching token price:", err);
            selectedTokenPrice = null;
        }
    }

    const handleTokenSelect = (token: TokenInfo) => {
        tokenAddress = token.address;
        selectedToken = token;
        isTokenDropdownOpen = false;
        fetchTokenPrice(token.address, token.decimals);
    };

    // Add function to fetch account balance
    async function fetchAccountBalance(publicKey: string) {
        try {
            const balance = await invoke("get_account_balance", { publicKey });
            selectedAccountBalance = Number(balance) / Math.pow(10, 9); // Convert from lamports to SOL
        } catch (err) {
            console.error("Error fetching account balance:", err);
            selectedAccountBalance = null;
        }
    }

    // Update account selection handler
    async function handleAccountSelect(event: Event) {
        const select = event.target as HTMLSelectElement;
        selectedAccountId = select.value;

        if (selectedAccountId) {
            const selectedAccount = accounts.find((acc) => acc.id.toString() === selectedAccountId);
            if (selectedAccount) {
                await fetchAccountBalance(selectedAccount.public_key);
            }
        } else {
            selectedAccountBalance = null;
        }
    }

    onMount(async () => {
        await Promise.all([loadStrategies(), loadAccounts(), loadTokens()]);
    });

    async function loadStrategies() {
        try {
            strategies = await invoke("get_all_strategies_with_full_information");
            console.log("STRATEGIES", strategies);
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
            if (!selectedToken) {
                throw new Error("Please select a token");
            }

            await invoke("add_strategy", {
                strategyType,
                intervalTime,
                accountPrivateKey: selectedAccount.private_key,
                tokenAddress,
                price: Math.trunc(price * Math.pow(10, 9)),
                amount:
                    strategyType === "Buy"
                        ? Math.trunc(amount * Math.pow(10, 9))
                        : Math.trunc(amount * Math.pow(10, selectedToken.decimals)),
                prioritizationFee,
                slippage: Math.trunc(slippage * 100),
            });

            await loadStrategies();
            showNotification(
                `Successfully added ${strategyType} strategy for ${selectedToken.name}`
            );
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
        intervalTime = 20;
        selectedAccountId = "";
        tokenAddress = "";
        price = 0;
        amount = 0;
        prioritizationFee = 5000;
        slippage = 5;
        error = null;
        selectedToken = null;
        isTokenDropdownOpen = false;
        selectedTokenPrice = null;
        selectedAccountBalance = null;
    }

    async function deleteStrategy(id: number) {
        try {
            await invoke("delete_strategy", { strategyId: id });
            await loadStrategies();
            showNotification("Strategy has been successfully deleted");
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

    async function toggleStrategyStatus(id: number) {
        try {
            await invoke("toggle_strategy_status", { strategyId: id });
            await loadStrategies();
            showNotification("Strategy status updated successfully");
        } catch (err) {
            console.error("Error toggling strategy status:", err);
            showNotification("Failed to update strategy status: " + err, true);
        }
    }
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
                        <th>Slippage</th>
                        <th>Status</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each strategies as strategy}
                        <tr>
                            <td>{strategy.strategyType}</td>
                            <td class="address-cell">
                                {#if strategy.tokenName}
                                    <div class="token-info">
                                        <div class="token-name-with-logo">
                                            <span class="token-name"
                                                >{removeNullChars(strategy.tokenName)}</span
                                            >
                                            {#if strategy.logoUri}
                                                <img
                                                    src={strategy.logoUri}
                                                    alt={strategy.tokenName}
                                                    class="token-logo"
                                                />
                                            {/if}
                                        </div>
                                        <span class="token-address"
                                            >({shortenAddress(strategy.tokenAddress)})</span
                                        >
                                    </div>
                                {:else}
                                    {shortenAddress(strategy.tokenAddress)}
                                {/if}
                            </td>
                            <td>{(strategy.price / Math.pow(10, 9)).toFixed(9)}</td>
                            <td>
                                {strategy.strategyType === "Buy"
                                    ? Number(
                                          (strategy.amount / Math.pow(10, 9)).toFixed(9)
                                      ).toString()
                                    : Number(
                                          (
                                              strategy.amount / Math.pow(10, strategy.decimals)
                                          ).toFixed(strategy.decimals)
                                      ).toString()}
                            </td><td>{strategy.intervalTime}s</td>
                            <td>{(strategy.slippage / 100).toFixed(2)}%</td>
                            <td>
                                <Tooltip
                                    text={strategy.status === "Executing"
                                        ? "Disable this strategy"
                                        : strategy.status === "Disabled"
                                          ? "Execute this strategy"
                                          : "Executed successfully, can't toggle"}
                                >
                                    <button
                                        class="status-badge status-{strategy.status.toLowerCase()}"
                                        on:click={() => toggleStrategyStatus(strategy.id)}
                                        disabled={strategy.status === "Executed"}
                                    >
                                        {strategy.status}
                                    </button>
                                </Tooltip>
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
                        <div class="helper-text">
                            {#if strategyType === "Buy"}
                                <span>Buy at a specific price</span>
                            {:else}
                                <span>Sell at a specific price</span>
                            {/if}
                        </div>
                    </div>

                    <div class="form-group">
                        <label for="account">Account</label>
                        <select
                            id="account"
                            class="select-input"
                            bind:value={selectedAccountId}
                            on:change={handleAccountSelect}
                            required
                        >
                            <option value="">Select an account</option>
                            {#each accounts as account}
                                <option value={account.id.toString()}>
                                    {account.name} ({shortenAddress(account.public_key)})
                                </option>
                            {/each}
                        </select>
                        {#if selectedAccountId}
                            <div class="helper-text">
                                {#if selectedAccountBalance !== null}
                                    <span>Balance: {selectedAccountBalance.toFixed(4)} SOL</span>
                                {:else}
                                    <span class="text-muted">Loading balance...</span>
                                {/if}
                            </div>
                        {/if}
                    </div>

                    <div class="form-group">
                        <label for="token-address">Token</label>
                        <div class="custom-select">
                            <div
                                class="select-header"
                                on:click={() => (isTokenDropdownOpen = !isTokenDropdownOpen)}
                            >
                                {#if selectedToken}
                                    <div class="token-item">
                                        <img
                                            src={selectedToken.logoUri}
                                            alt={selectedToken.symbol}
                                            class="token-logo"
                                        />
                                        <span
                                            >{removeNullChars(selectedToken.name)} ({shortenAddress(
                                                selectedToken.address
                                            )})</span
                                        >
                                    </div>
                                {:else}
                                    <span>Select a token</span>
                                {/if}
                                <span class="dropdown-arrow">▼</span>
                            </div>

                            {#if isTokenDropdownOpen}
                                <div class="dropdown-options">
                                    {#each tokens.filter((token) => token.logoUri) as token}
                                        <div
                                            class="token-item"
                                            on:click={() => handleTokenSelect(token)}
                                        >
                                            <img
                                                src={token.logoUri}
                                                alt={token.symbol}
                                                class="token-logo"
                                            />
                                            <span
                                                >{removeNullChars(token.name)} ({shortenAddress(
                                                    token.address
                                                )})</span
                                            >
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </div>

                        {#if selectedToken}
                            <div class="helper-text">
                                {#if selectedTokenPrice !== null}
                                    <span>Current price: {selectedTokenPrice.toFixed(9)} SOL</span>
                                {:else}
                                    <span class="text-muted">Loading price...</span>
                                {/if}
                            </div>
                        {/if}
                    </div>

                    <div class="form-group">
                        <label for="interval-time">
                            <div class="label-with-tooltip">
                                Interval Time (seconds)
                                <Tooltip
                                    text="The interval period between each times the strategy executes"
                                >
                                    <span class="tooltip-trigger">
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
                                            <circle cx="12" cy="12" r="10"></circle>
                                            <line x1="12" y1="16" x2="12" y2="12"></line>
                                            <line x1="12" y1="8" x2="12.01" y2="8"></line>
                                        </svg>
                                    </span>
                                </Tooltip>
                            </div>
                        </label>
                        <input
                            type="number"
                            id="interval-time"
                            bind:value={intervalTime}
                            min="1"
                            required
                        />
                    </div>

                    <div class="form-row">
                        <div class="form-group half-width">
                            <label for="price">Price (SOL)</label>
                            <input
                                type="number"
                                id="price"
                                class="number-input"
                                bind:value={price}
                                min="0"
                                step="any"
                            />
                        </div>
                        <div class="form-group half-width">
                            <label for="amount">
                                {#if strategyType === "Buy"}
                                    Amount SOL to buy
                                {:else}
                                    Amount {selectedToken?.name || ""} to sell
                                {/if}
                            </label>
                            <input
                                type="number"
                                id="amount"
                                class="number-input"
                                bind:value={amount}
                                min="0"
                                step="any"
                            />
                        </div>
                    </div>

                    <div class="form-row">
                        <div class="form-group half-width">
                            <label for="prioritization-fee">Prioritization Fee</label>
                            <input
                                type="number"
                                id="prioritization-fee"
                                class="number-input"
                                bind:value={prioritizationFee}
                                min="0"
                                required
                            />
                        </div>
                        <div class="form-group half-width">
                            <label for="slippage">Slippage (%)</label>
                            <input
                                type="number"
                                id="slippage"
                                class="number-input"
                                bind:value={slippage}
                                min="0"
                                max="100"
                                step="any"
                                required
                            />
                        </div>
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
        <div class="notification-container">
            <div class="notification {notification.isError ? 'error' : 'success'}" role="alert">
                <div class="notification-content">
                    <span class="notification-icon">
                        {#if notification.isError}
                            ⚠️
                        {:else}
                            ✅
                        {/if}
                    </span>
                    <span class="notification-message">{notification.message}</span>
                </div>
            </div>
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
                        <button
                            type="button"
                            class="dialog-button cancel-button"
                            on:click={closeDeleteConfirm}
                        >
                            Cancel
                        </button>
                        <button
                            type="button"
                            class="dialog-button delete-button-confirm"
                            on:click={confirmDelete}
                        >
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
        box-sizing: border-box;
    }

    .form-group {
        margin-bottom: 0.75rem;
        width: 100%;
        box-sizing: border-box;
    }

    .form-group label {
        display: block;
        margin-bottom: 0.25rem;
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
        padding: 1rem;
        box-sizing: border-box;
    }

    .dialog {
        background-color: white;
        border-radius: 12px;
        width: 90%;
        max-width: 600px;
        display: flex;
        flex-direction: column;
        box-shadow:
            0 4px 6px -1px rgba(0, 0, 0, 0.1),
            0 2px 4px -1px rgba(0, 0, 0, 0.06);
    }

    .dialog-header {
        padding: 1rem 1.25rem;
        border-bottom: 1px solid #e5e7eb;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .dialog-header h2 {
        font-size: 1.25rem;
        font-weight: 600;
        color: #111827;
        margin: 0;
    }

    .dialog-content {
        padding: 1rem;
    }

    .dialog-content p {
        margin: 0;
        color: #4b5563;
        font-size: 1rem;
        line-height: 1.5;
    }

    .dialog-actions {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
        margin-top: 1rem;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 1.5rem;
        color: #6b7280;
        cursor: pointer;
        padding: 0.25rem;
        line-height: 1;
    }

    .close-button:hover {
        color: #374151;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .dialog {
            background-color: #1f2937;
            border: 1px solid #374151;
        }

        .dialog-header {
            border-bottom-color: #374151;
        }

        .dialog-header h2 {
            color: #f3f4f6;
        }

        .dialog-content p {
            color: #d1d5db;
        }

        .close-button {
            color: #9ca3af;
        }

        .close-button:hover {
            color: #f3f4f6;
        }
    }

    .cancel-button {
        padding: 0.75rem 1.5rem;
        border-radius: 8px;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        background-color: white;
        border: 1px solid #e5e7eb;
        color: #4b5563;
        display: inline-flex;
        align-items: center;
        justify-content: center;
    }

    .cancel-button:hover {
        background-color: #f3f4f6;
        border-color: #d1d5db;
    }

    .submit-button {
        padding: 0.75rem 1.5rem;
        border-radius: 8px;
        font-size: 1rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        background-color: #396cd8;
        border: none;
        color: white;
        box-shadow: 0 2px 4px rgba(57, 108, 216, 0.2);
    }

    .submit-button:hover {
        background-color: #2857b8;
        transform: translateY(-1px);
        box-shadow: 0 4px 8px rgba(57, 108, 216, 0.3);
    }

    .submit-button:active {
        transform: translateY(0);
    }

    .submit-button:disabled {
        background-color: #93a8d5;
        cursor: not-allowed;
        transform: none;
        box-shadow: none;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .cancel-button {
            background-color: #374151;
            border-color: #4b5563;
            color: #e5e7eb;
        }

        .cancel-button:hover {
            background-color: #4b5563;
            border-color: #6b7280;
        }

        .submit-button {
            background-color: #4a7be0;
        }

        .submit-button:hover {
            background-color: #3967c4;
        }

        .submit-button:disabled {
            background-color: #374151;
            color: #9ca3af;
        }
    }

    /* Add loading spinner for submit button */
    .loader {
        width: 16px;
        height: 16px;
        border: 2px solid #ffffff;
        border-bottom-color: transparent;
        border-radius: 50%;
        display: inline-block;
        animation: rotation 1s linear infinite;
    }

    @keyframes rotation {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
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

    .custom-select {
        position: relative;
        width: 100%;
        box-sizing: border-box;
    }

    .select-header {
        width: 100%;
        padding: 0.5rem 0.75rem;
        border: 1px solid #ddd;
        border-radius: 8px;
        background-color: white;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: space-between;
        box-sizing: border-box;
    }

    .dropdown-arrow {
        margin-left: 8px;
        color: #666;
    }

    .dropdown-options {
        position: absolute;
        top: 100%;
        left: 0;
        width: calc(100% - 2px);
        background-color: white;
        border: 1px solid #ddd;
        border-radius: 8px;
        margin-top: 0.25rem;
        max-height: 200px;
        overflow-y: auto;
        z-index: 1000;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        box-sizing: border-box;
    }

    .token-item {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 0.5rem 0.75rem;
        cursor: pointer;
        width: 100%;
        box-sizing: border-box;
    }

    .token-item:hover {
        background-color: #f3f4f6;
    }

    .token-logo {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        object-fit: cover;
    }

    @media (prefers-color-scheme: dark) {
        .select-header {
            background-color: #374151;
            border-color: #4b5563;
            color: #f3f4f6;
        }

        .dropdown-options {
            background-color: #374151;
            border-color: #4b5563;
        }

        .token-item:hover {
            background-color: #4b5563;
        }

        .dropdown-arrow {
            color: #9ca3af;
        }
    }

    .helper-text {
        margin-top: 0.25rem;
        font-size: 0.875rem;
        color: #666;
    }

    .text-muted {
        color: #666;
        font-style: italic;
    }

    @media (prefers-color-scheme: dark) {
        .helper-text {
            color: #9ca3af;
        }

        .text-muted {
            color: #9ca3af;
        }
    }

    /* Add text overflow handling */
    .token-item span {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .form-group {
        margin-bottom: 0.75rem;
        width: 100%;
        position: relative;
        box-sizing: border-box;
    }

    /* Hide spinner buttons for number inputs */
    input[type="number"]::-webkit-inner-spin-button,
    input[type="number"]::-webkit-outer-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    input[type="number"] {
        -moz-appearance: textfield; /* Firefox */
    }

    /* Style the number inputs like other inputs */
    input[type="number"] {
        width: 100%;
        padding: 0.8rem;
        border: 1px solid #ddd;
        border-radius: 8px;
        font-size: 1rem;
        box-sizing: border-box;
    }

    @media (prefers-color-scheme: dark) {
        input[type="number"] {
            background-color: #374151;
            border-color: #4b5563;
            color: #f3f4f6;
        }
    }

    .label-with-tooltip {
        display: flex;
        align-items: baseline;
        gap: 0.5rem;
    }

    .tooltip-trigger {
        display: inline-flex;
        align-items: center;
        color: #666;
        cursor: help;
        position: relative;
        top: 3px;
    }

    @media (prefers-color-scheme: dark) {
        .tooltip-trigger {
            color: #9ca3af;
        }
    }

    /* Add these styles for the strategies table */
    .strategies-container {
        margin-top: 2rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 12px;
        overflow: hidden;
    }

    .strategies-table {
        width: 100%;
        border-collapse: collapse;
        text-align: left;
    }

    .strategies-table th,
    .strategies-table td {
        padding: 1rem;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }

    .strategies-table th {
        background: rgba(255, 255, 255, 0.05);
        font-weight: 600;
        color: #374151;
        padding: 1rem;
        text-align: left;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }

    .address-cell {
        font-family: monospace;
        font-size: 0.9rem;
    }

    .empty-state {
        text-align: center;
        padding: 3rem;
        color: #666;
    }

    /* Make table responsive */
    @media (max-width: 768px) {
        .strategies-container {
            overflow-x: auto;
        }

        .strategies-table {
            min-width: 600px;
        }
    }

    /* Status badge styles - keep existing but adjust colors */
    .status-badge {
        padding: 0.25rem 0.75rem;
        border-radius: 9999px;
        font-size: 0.875rem;
        font-weight: 500;
        border: none;
        cursor: pointer;
        transition: opacity 0.2s;
    }

    .status-badge:disabled {
        cursor: not-allowed;
        opacity: 0.7;
    }

    .status-badge:not(:disabled):hover {
        opacity: 0.8;
    }

    .status-executing {
        background-color: #10b981;
        color: white;
    }

    .status-disabled {
        background-color: #6b7280;
        color: white;
    }

    .status-executed {
        background-color: #3b82f6;
        color: white;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .strategies-table th {
            color: #e5e7eb;
        }

        .strategies-table td {
            border-bottom-color: rgba(255, 255, 255, 0.05);
        }

        .empty-state {
            color: #999;
        }

        .status-executing {
            background-color: #065f46;
            color: #d1fae5;
        }

        .status-executed {
            background-color: #075985;
            color: #e0f2fe;
        }

        .status-disabled {
            background-color: #991b1b;
            color: #fee2e2;
        }
    }

    .token-info {
        display: flex;
        flex-direction: column;
        gap: 2px;
    }

    .token-name {
        font-weight: 500;
        color: #374151;
    }

    .token-address {
        font-size: 0.8rem;
        color: #666;
    }

    @media (prefers-color-scheme: dark) {
        .token-name {
            color: #e5e7eb;
        }

        .token-address {
            color: #9ca3af;
        }
    }

    .token-name-with-logo {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .token-logo {
        width: 20px;
        height: 20px;
        border-radius: 50%;
    }

    /* Update the action buttons and delete button styles */
    .action-buttons {
        display: flex;
        gap: 0.5rem;
        justify-content: flex-start;
        align-items: center;
    }

    .icon-button {
        background: none;
        border: none;
        padding: 0.5rem;
        cursor: pointer;
        color: #666;
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s;
    }

    .icon-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: #396cd8;
    }

    .delete-button {
        color: #dc2626;
    }

    .delete-button:hover {
        background: rgba(220, 38, 38, 0.1);
        color: #ef4444;
    }

    @media (prefers-color-scheme: dark) {
        .icon-button {
            color: #999;
        }

        .icon-button:hover {
            background: rgba(255, 255, 255, 0.05);
            color: #4a7be0;
        }

        .delete-button {
            color: #ef4444;
        }

        .delete-button:hover {
            background: rgba(239, 68, 68, 0.1);
            color: #f87171;
        }
    }

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
        padding: 1rem;
        box-sizing: border-box;
    }

    .popup-content {
        background-color: white;
        border-radius: 12px;
        width: 90%;
        max-width: 400px;
        display: flex;
        flex-direction: column;
        box-shadow:
            0 4px 6px -1px rgba(0, 0, 0, 0.1),
            0 2px 4px -1px rgba(0, 0, 0, 0.06);
    }

    .popup-content h2 {
        font-size: 1.25rem;
        font-weight: 600;
        color: #111827;
        margin: 0;
    }

    .popup-content p {
        margin: 0;
        color: #4b5563;
        font-size: 1rem;
        line-height: 1.5;
    }

    .button-group {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        margin-top: 1.5rem;
        padding: 0 1.5rem 1.5rem;
    }

    .popup-button {
        background: none;
        border: none;
        font-size: 1.5rem;
        color: #6b7280;
        cursor: pointer;
        padding: 0.25rem;
        line-height: 1;
    }

    .popup-button:hover {
        color: #374151;
    }

    .popup-button.cancel-button {
        background-color: transparent;
        border: none;
        color: #6b7280;
    }

    .popup-button.cancel-button:hover {
        color: #374151;
    }

    .popup-button.delete-button-confirm {
        background-color: #dc2626;
        color: white;
    }

    .popup-button.delete-button-confirm:hover {
        background-color: #b91c1c;
    }

    .dialog-button {
        padding: 0.875rem 1.75rem;
        border-radius: 8px;
        font-size: 1rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
    }

    .dialog-button.cancel-button {
        background-color: white;
        border: 1px solid #e5e7eb;
        color: #4b5563;
    }

    .dialog-button.cancel-button:hover {
        background-color: #f3f4f6;
        border-color: #d1d5db;
    }

    .dialog-button.delete-button-confirm {
        background-color: #dc2626;
        border: none;
        color: white;
    }

    .dialog-button.delete-button-confirm:hover {
        background-color: #b91c1c;
    }

    @media (prefers-color-scheme: dark) {
        .dialog-button.cancel-button {
            background-color: #374151;
            border-color: #4b5563;
            color: #e5e7eb;
        }

        .dialog-button.cancel-button:hover {
            background-color: #4b5563;
            border-color: #6b7280;
        }

        .dialog-button.delete-button-confirm {
            background-color: #dc2626;
        }

        .dialog-button.delete-button-confirm:hover {
            background-color: #b91c1c;
        }
    }

    .form-row {
        display: flex;
        gap: 1rem;
        margin-bottom: 0.75rem;
        width: 100%;
    }

    .half-width {
        flex: 1;
        margin-bottom: 0; /* Override the default margin-bottom from form-group */
    }

    .notification-container {
        position: fixed;
        bottom: 24px;
        right: 24px;
        z-index: 1000;
    }

    .notification {
        min-width: 300px;
        padding: 16px;
        border-radius: 8px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        background: white;
        margin-top: 12px;
        animation: slideIn 0.3s ease-out;
    }

    .notification-content {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .notification-icon {
        margin-right: 8px;
        font-size: 1.2rem;
    }

    .notification-message {
        color: #374151;
        font-size: 0.875rem;
        line-height: 1.25rem;
        font-weight: 500;
    }

    @media (prefers-color-scheme: dark) {
        .notification {
            background: #1f2937;
        }

        .notification-message {
            color: #f3f4f6;
        }
    }
</style>
