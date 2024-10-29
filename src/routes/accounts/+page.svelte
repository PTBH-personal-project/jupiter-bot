<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { privateKeyToPublicKey } from "../../utils/account_utils";
    import type { Account } from "../../types/accounts";

    let showPopup = false;
    let privateKeyOrPath = "";
    let accountName = "";
    let publicKey: string | null = "";
    let accounts: Account[] = [];

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
        console.log("Importing account:", { accountName, privateKeyOrPath });
        try {
            await invoke("import_account", { accountName, privateKey: privateKeyOrPath });
            await loadAccounts();
            showPopup = false;
            privateKeyOrPath = "";
            accountName = "";
        } catch (error) {
            console.error("Error importing account:", error);
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
                    </tr>
                </thead>
                <tbody>
                    {#each accounts as account}
                        <tr>
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
                <textarea bind:value={accountName} placeholder="Your account name" rows="1"
                ></textarea>
                <textarea
                    bind:value={privateKeyOrPath}
                    placeholder="Paste your private key or path to private key here"
                    rows="2"
                    on:input={previewPublicKey}
                ></textarea>
                <div class="readonly-textarea">
                    {publicKey ? `Public key: ${publicKey}` : "Provided secret key is not valid"}
                </div>
                <div class="button-group">
                    <button type="button" class="popup-button cancel-button" on:click={togglePopup}
                        >Cancel</button
                    >
                    <button type="submit" class="popup-button submit-button">Submit</button>
                </div>
            </form>
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
</style>
