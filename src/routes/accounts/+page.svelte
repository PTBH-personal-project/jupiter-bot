<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { privateKeyToPublicKey } from "../../utils/account_utils";
    import type { Account } from "../../types/accounts";

    let showPopup = false;
    let privateKeyOrPath = "";
    let accountName = "";
    let accountDescription = "";
    let publicKey: string | null = "";
    let accounts: Account[] = [];

    let showDeleteConfirmDialog = false;
    let accountToDelete: number | null = null;

    async function loadAccounts() {
        try {
            console.log("Start loading accounts...");
            accounts = await invoke("get_accounts");
            console.log("Loaded accounts:", accounts);
        } catch (error) {
            console.error("Error loading accounts:", error);
        }
    }

    onMount(async () => {
        console.log("Start onMount");
        await loadAccounts();
    });

    function togglePopup() {
        showPopup = !showPopup;
    }

    async function previewPublicKey() {
        const publicKeyResolved = await privateKeyToPublicKey(privateKeyOrPath);
        publicKey = publicKeyResolved ? publicKeyResolved.toBase58() : null;
    }

    async function handleSubmit() {
        console.log("Importing account:", { accountName, privateKeyOrPath, accountDescription });
        try {
            await invoke("import_account", {
                accountName,
                privateKey: privateKeyOrPath,
                description: accountDescription,
            });
            await loadAccounts();
            showPopup = false;
            privateKeyOrPath = "";
            accountName = "";
            accountDescription = "";
        } catch (error) {
            console.error("Error importing account:", error);
        }
    }

    async function deleteAccount(id: number) {
        console.log("Attempting to delete account:", id);
        accountToDelete = id;
        showDeleteConfirmDialog = true;
    }

    async function confirmDelete() {
        if (accountToDelete !== null) {
            try {
                console.log("Deleting account with ID:", accountToDelete);
                await invoke("delete_account", { accountId: accountToDelete });
                console.log("Account deleted successfully");
                await loadAccounts();
            } catch (error) {
                console.error("Error deleting account:", error);
            } finally {
                showDeleteConfirmDialog = false;
                accountToDelete = null;
            }
        }
    }

    function cancelDelete() {
        showDeleteConfirmDialog = false;
        accountToDelete = null;
        console.log("Account deletion cancelled by user");
    }

    async function toggleAccountStatus(accountId: number, currentStatus: string) {
        try {
            const newStatus = currentStatus.toLowerCase() === "enabled" ? "Disabled" : "Enabled";
            console.log(`Toggling account ${accountId} status to ${newStatus}`);
            await invoke("toggle_account_status", { accountId, newStatus });
            await loadAccounts();
        } catch (error) {
            console.error("Error toggling account status:", error);
        }
    }
</script>

<div class="accounts-container">
    <h1>Accounts</h1>

    <button class="import-button" on:click={togglePopup}>Import Account</button>

    {#if accounts.length > 0}
        <div class="accounts-list">
            <h3>Your Accounts</h3>
            <table>
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Public Key</th>
                        <th>Private Key</th>
                        <th>Description</th>
                        <th>Status</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {#each accounts as account}
                        <tr
                            class={account.status.toLowerCase() === "disabled"
                                ? "disabled-row"
                                : ""}
                        >
                            <td>{account.id}</td>
                            <td>{account.name}</td>
                            <td class="key-cell" title={account.public_key}>
                                {account.public_key.slice(0, 8)}...{account.public_key.slice(-8)}
                            </td>
                            <td class="key-cell" title={account.private_key}>
                                {account.private_key.slice(0, 8)}...{account.private_key.slice(-8)}
                            </td>
                            <td>{account.description || "-"}</td>
                            <td>
                                <span class="status-badge status-{account.status.toLowerCase()}">
                                    {account.status}
                                </span>
                            </td>
                            <td>
                                <div class="action-buttons">
                                    <div class="tooltip-container">
                                        <button
                                            class="status-toggle-button"
                                            on:click={() =>
                                                toggleAccountStatus(account.id, account.status)}
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
                                                    d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
                                                />
                                            </svg>
                                        </button>
                                        <span class="tooltip">
                                            {account.status.toLowerCase() === "enabled"
                                                ? "Disable Account"
                                                : "Enable Account"}
                                        </span>
                                    </div>

                                    <div class="tooltip-container">
                                        <button
                                            class="delete-button"
                                            on:click={() => deleteAccount(account.id)}
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
                                                <circle cx="12" cy="12" r="10" />
                                                <line x1="15" y1="9" x2="9" y2="15" />
                                                <line x1="9" y1="9" x2="15" y2="15" />
                                            </svg>
                                        </button>
                                        <span class="tooltip">Delete Account</span>
                                    </div>
                                </div>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {:else}
        <p class="no-accounts">No accounts found. Import an account to get started.</p>
    {/if}
</div>

{#if showPopup}
    <div class="popup-overlay">
        <div class="popup-content">
            <h2>Import Account</h2>
            <form on:submit|preventDefault={handleSubmit}>
                <div class="form-group">
                    <label for="account-name">Account Name</label>
                    <textarea
                        id="account-name"
                        bind:value={accountName}
                        placeholder="Enter account name"
                        rows="1"
                    ></textarea>
                </div>

                <div class="form-group">
                    <label for="private-key">Private Key</label>
                    <textarea
                        id="private-key"
                        bind:value={privateKeyOrPath}
                        placeholder="Paste your private key or path to private key here"
                        rows="2"
                        on:input={previewPublicKey}
                    ></textarea>
                </div>

                <div class="form-group">
                    <label for="description">Description (optional)</label>
                    <textarea
                        id="description"
                        bind:value={accountDescription}
                        placeholder="Enter account description"
                        rows="2"
                    ></textarea>
                </div>

                <div class="readonly-textarea">
                    {publicKey ? `Public key: ${publicKey}` : "Provided secret key is not valid"}
                </div>

                <div class="button-group">
                    <button type="button" class="popup-button cancel-button" on:click={togglePopup}>
                        Cancel
                    </button>
                    <button type="submit" class="popup-button submit-button"> Submit </button>
                </div>
            </form>
        </div>
    </div>
{/if}

{#if showDeleteConfirmDialog}
    <div class="popup-overlay">
        <div class="popup-content confirm-dialog">
            <h2>Confirm Deletion</h2>
            <p>Are you sure you want to delete this account?</p>
            <div class="button-group">
                <button class="popup-button cancel-button" on:click={cancelDelete}> Cancel </button>
                <button class="popup-button delete-button-confirm" on:click={confirmDelete}>
                    Delete
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .accounts-container {
        padding: 20px;
    }

    .import-button {
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

    .import-button:hover {
        background-color: #2857b8;
        transform: translateY(-1px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    }

    .import-button:active {
        transform: translateY(0);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    @media (prefers-color-scheme: dark) {
        .import-button {
            background-color: #4a7be0;
        }

        .import-button:hover {
            background-color: #3967c4;
        }
    }

    .accounts-list {
        margin-top: 20px;
        overflow-x: auto;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 10px;
        min-width: 800px;
    }

    th,
    td {
        padding: 12px;
        text-align: left;
        border-bottom: 1px solid #ddd;
    }

    th {
        background-color: #f5f5f5;
        font-weight: bold;
    }

    .key-cell {
        font-family: monospace;
        font-size: 0.9em;
        cursor: pointer;
    }

    .status-badge {
        padding: 4px 8px;
        border-radius: 12px;
        font-size: 0.85em;
        font-weight: 500;
    }

    .status-enabled {
        background-color: #e6f4ea;
        color: #1e7e34;
    }

    .status-disabled {
        background-color: #feeced;
        color: #dc3545;
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
    }

    .popup-content {
        background-color: #fff;
        padding: 20px;
        border-radius: 8px;
        width: 80%;
        max-width: 500px;
        max-height: 90vh;
        overflow-y: auto;
    }

    textarea {
        width: 100%;
        padding: 10px;
        margin-bottom: 10px;
        border-radius: 8px;
        border: 1px solid #ccc;
        font-family: inherit;
        font-size: 1em;
        box-sizing: border-box;
    }

    .readonly-textarea {
        width: 100%;
        padding: 10px;
        margin-bottom: 10px;
        font-size: 0.875em;
        color: #666;
        background-color: #f5f5f5;
        border-radius: 8px;
    }

    .button-group {
        display: flex;
        justify-content: flex-end;
        gap: 15px;
        margin-top: 20px;
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

    .submit-button {
        background-color: #396cd8;
        color: white;
    }

    .submit-button:hover {
        background-color: #2857b8;
        transform: translateY(-1px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    }

    .submit-button:active {
        transform: translateY(0);
        background-color: #214a9c;
    }

    .cancel-button {
        background-color: #e0e0e0;
        color: #333;
    }

    .cancel-button:hover {
        background-color: #d0d0d0;
        transform: translateY(-1px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    }

    .cancel-button:active {
        transform: translateY(0);
        background-color: #c0c0c0;
    }

    @media (prefers-color-scheme: dark) {
        th {
            background-color: #2f2f2f;
        }

        td {
            border-bottom-color: #444;
        }

        .popup-content {
            background-color: #2f2f2f;
            color: #f6f6f6;
        }

        textarea {
            background-color: #1f1f1f;
            color: #f6f6f6;
            border-color: #444;
        }

        .readonly-textarea {
            background-color: #1f1f1f;
            color: #bbb;
        }

        .status-enabled {
            background-color: #1e7e34;
            color: #e6f4ea;
        }

        .status-disabled {
            background-color: #dc3545;
            color: #feeced;
        }

        .submit-button {
            background-color: #4a7be0;
        }

        .submit-button:hover {
            background-color: #3967c4;
        }

        .submit-button:active {
            background-color: #2d539e;
        }

        .cancel-button {
            background-color: #3f3f3f;
            color: #f0f0f0;
        }

        .cancel-button:hover {
            background-color: #4f4f4f;
        }

        .cancel-button:active {
            background-color: #2f2f2f;
        }
    }

    .delete-button {
        background: none;
        border: none;
        padding: 8px;
        cursor: pointer;
        color: #666;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease-in-out;
    }

    .delete-button:hover {
        color: #dc3545;
        background-color: rgba(220, 53, 69, 0.1);
    }

    .delete-button:active {
        transform: scale(0.95);
    }

    @media (prefers-color-scheme: dark) {
        .delete-button {
            color: #999;
        }

        .delete-button:hover {
            color: #ff4d4d;
            background-color: rgba(255, 77, 77, 0.1);
        }
    }

    .tooltip-container {
        position: relative;
        display: inline-block;
    }

    .tooltip {
        visibility: hidden;
        position: absolute;
        background-color: #333;
        color: white;
        text-align: center;
        padding: 5px 10px;
        border-radius: 6px;
        font-size: 0.8em;
        white-space: nowrap;

        /* Position the tooltip */
        bottom: 100%;
        left: 50%;
        transform: translateX(-50%);
        margin-bottom: 5px;

        /* Add transition */
        opacity: 0;
        transition:
            opacity 0.2s,
            visibility 0.2s;

        /* Add a small triangle */
        &::after {
            content: "";
            position: absolute;
            top: 100%;
            left: 50%;
            margin-left: -5px;
            border-width: 5px;
            border-style: solid;
            border-color: #333 transparent transparent transparent;
        }
    }

    .tooltip-container:hover .tooltip {
        visibility: visible;
        opacity: 1;
    }

    @media (prefers-color-scheme: dark) {
        .tooltip {
            background-color: #666;
        }

        .tooltip::after {
            border-color: #666 transparent transparent transparent;
        }
    }

    .confirm-dialog {
        max-width: 400px;
    }

    .delete-button-confirm {
        background-color: #dc3545;
        color: white;
    }

    .delete-button-confirm:hover {
        background-color: #c82333;
    }

    @media (prefers-color-scheme: dark) {
        .delete-button-confirm {
            background-color: #dc3545;
        }

        .delete-button-confirm:hover {
            background-color: #bd2130;
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
    }

    .popup-content {
        background-color: #fff;
        padding: 20px;
        border-radius: 8px;
        width: 80%;
        max-height: 90vh;
        overflow-y: auto;
    }

    .button-group {
        display: flex;
        justify-content: flex-end;
        gap: 15px;
        margin-top: 20px;
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

    @media (prefers-color-scheme: dark) {
        .popup-content {
            background-color: #2f2f2f;
            color: #f6f6f6;
        }
    }

    .form-group {
        margin-bottom: 16px;
    }

    .form-group label {
        display: block;
        margin-bottom: 8px;
        font-weight: 500;
        color: #333;
    }

    textarea {
        width: 100%;
        padding: 10px;
        border-radius: 8px;
        border: 1px solid #ccc;
        font-family: inherit;
        font-size: 1em;
        box-sizing: border-box;
        resize: vertical;
    }

    .readonly-textarea {
        width: 100%;
        padding: 10px;
        margin-bottom: 20px;
        font-size: 0.875em;
        color: #666;
        background-color: #f5f5f5;
        border-radius: 8px;
    }

    @media (prefers-color-scheme: dark) {
        .form-group label {
            color: #f0f0f0;
        }

        textarea {
            background-color: #1f1f1f;
            color: #f6f6f6;
            border-color: #444;
        }

        textarea::placeholder {
            color: #888;
        }

        .readonly-textarea {
            background-color: #2f2f2f;
            color: #bbb;
        }
    }

    .action-buttons {
        display: flex;
        gap: 8px;
        align-items: center;
    }

    .status-toggle-button {
        background: none;
        border: none;
        padding: 8px;
        cursor: pointer;
        color: #666;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease-in-out;
    }

    .status-toggle-button:hover {
        color: #396cd8;
        background-color: rgba(57, 108, 216, 0.1);
    }

    .status-toggle-button:active {
        transform: scale(0.95);
    }

    @media (prefers-color-scheme: dark) {
        .status-toggle-button {
            color: #999;
        }

        .status-toggle-button:hover {
            color: #4a7be0;
            background-color: rgba(74, 123, 224, 0.1);
        }
    }

    .disabled-row {
        background-color: rgba(0, 0, 0, 0.05);
    }

    .disabled-row td {
        opacity: 0.7;
    }

    @media (prefers-color-scheme: dark) {
        .disabled-row {
            background-color: rgba(0, 0, 0, 0.2);
        }

        .disabled-row td {
            opacity: 0.6;
        }
    }
</style>
