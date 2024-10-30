<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    interface RpcEndpoint {
        id: number;
        url: string;
        name: string;
        description: string;
    }

    let rpcEndpoints: RpcEndpoint[] = [];
    let loading = true;
    let isEditing = false;
    let editedRpc: RpcEndpoint | null = null;
    let showImportPopup = false;
    let newRpc = {
        name: "",
        url: "",
        description: "",
    };
    let showDeleteConfirmDialog = false;
    let rpcToDelete: number | null = null;

    async function loadRpcEndpoints() {
        try {
            loading = true;
            rpcEndpoints = await invoke("get_rpcs_endpoint");
        } catch (error) {
            console.error("Error loading RPC endpoints:", error);
        } finally {
            loading = false;
        }
    }

    function startEditing(rpc: RpcEndpoint) {
        editedRpc = { ...rpc };
        isEditing = true;
    }

    function cancelEditing() {
        editedRpc = null;
        isEditing = false;
    }

    async function saveRpc() {
        if (!editedRpc) return;
        try {
            await invoke("update_rpc", {
                rpcId: editedRpc.id,
                name: editedRpc.name,
                url: editedRpc.url,
                description: editedRpc.description,
            });
            await loadRpcEndpoints();
            cancelEditing();
        } catch (error) {
            console.error("Error updating RPC:", error);
        }
    }

    async function importRpc() {
        try {
            await invoke("import_rpc", {
                name: newRpc.name,
                url: newRpc.url,
                description: newRpc.description,
            });
            await loadRpcEndpoints();
            showImportPopup = false;
            newRpc = {
                name: "",
                url: "",
                description: "",
            };
        } catch (error) {
            console.error("Error importing RPC:", error);
        }
    }

    async function deleteRpc(id: number) {
        rpcToDelete = id;
        showDeleteConfirmDialog = true;
    }

    async function confirmDelete() {
        if (rpcToDelete !== null) {
            try {
                await invoke("delete_rpc", { rpcId: rpcToDelete });
                await loadRpcEndpoints();
            } catch (error) {
                console.error("Error deleting RPC:", error);
            } finally {
                showDeleteConfirmDialog = false;
                rpcToDelete = null;
            }
        }
    }

    function cancelDelete() {
        showDeleteConfirmDialog = false;
        rpcToDelete = null;
    }

    onMount(() => {
        loadRpcEndpoints();
    });
</script>

<div class="container">
    <h1>RPC Endpoints</h1>
    <div class="import-section">
        <button class="import-button" on:click={() => (showImportPopup = true)}>
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
                <line x1="12" y1="5" x2="12" y2="19" />
                <line x1="5" y1="12" x2="19" y2="12" />
            </svg>
            Import RPC
        </button>
    </div>

    {#if loading}
        <div class="loading">Loading RPC endpoints...</div>
    {:else}
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>URL</th>
                    <th>Description</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each rpcEndpoints as rpc}
                    <tr>
                        <td>{rpc.name}</td>
                        <td class="url-cell">{rpc.url}</td>
                        <td>{rpc.description}</td>
                        <td>
                            <div class="action-buttons">
                                <div class="tooltip-container">
                                    <button class="icon-button" on:click={() => startEditing(rpc)}>
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
                                                d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
                                            />
                                            <path
                                                d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
                                            />
                                        </svg>
                                    </button>
                                    <span class="tooltip">Edit RPC</span>
                                </div>
                                <div class="tooltip-container">
                                    <button
                                        class="icon-button delete-button"
                                        on:click={() => deleteRpc(rpc.id)}
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
                                            <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
                                            <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
                                        </svg>
                                    </button>
                                    <span class="tooltip">Delete RPC</span>
                                </div>
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}

    {#if isEditing && editedRpc}
        <div class="modal-overlay">
            <div class="modal">
                <h2>Edit RPC Endpoint</h2>
                <div class="form-group">
                    <label for="name">Name</label>
                    <input type="text" id="name" bind:value={editedRpc.name} />
                </div>
                <div class="form-group">
                    <label for="url">URL</label>
                    <input type="text" id="url" bind:value={editedRpc.url} />
                </div>
                <div class="form-group">
                    <label for="description">Description</label>
                    <input type="text" id="description" bind:value={editedRpc.description} />
                </div>
                <div class="modal-buttons">
                    <button class="cancel-button" on:click={cancelEditing}>Cancel</button>
                    <button class="save-button" on:click={saveRpc}>Save</button>
                </div>
            </div>
        </div>
    {/if}

    {#if showImportPopup}
        <div class="modal-overlay">
            <div class="modal">
                <h2>Import RPC Endpoint</h2>
                <div class="form-group">
                    <label for="import-name">Name</label>
                    <input
                        type="text"
                        id="import-name"
                        bind:value={newRpc.name}
                        placeholder="Enter RPC name"
                    />
                </div>
                <div class="form-group">
                    <label for="import-url">URL</label>
                    <input
                        type="text"
                        id="import-url"
                        bind:value={newRpc.url}
                        placeholder="Enter RPC URL"
                    />
                </div>
                <div class="form-group">
                    <label for="import-description">Description</label>
                    <input
                        type="text"
                        id="import-description"
                        bind:value={newRpc.description}
                        placeholder="Enter RPC description"
                    />
                </div>
                <div class="modal-buttons">
                    <button class="cancel-button" on:click={() => (showImportPopup = false)}>
                        Cancel
                    </button>
                    <button
                        class="save-button"
                        on:click={importRpc}
                        disabled={!newRpc.name || !newRpc.url}
                    >
                        Import
                    </button>
                </div>
            </div>
        </div>
    {/if}

    {#if showDeleteConfirmDialog}
        <div class="modal-overlay">
            <div class="modal confirm-modal">
                <h2>Confirm Delete</h2>
                <p>Are you sure you want to delete this RPC endpoint?</p>
                <div class="modal-buttons">
                    <button class="cancel-button" on:click={cancelDelete}>Cancel</button>
                    <button class="delete-button confirm-delete" on:click={confirmDelete}>
                        Delete
                    </button>
                </div>
            </div>
        </div>
    {/if}
</div>

<style>
    .container {
        padding: 20px;
    }

    h1 {
        margin-bottom: 20px;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 20px;
    }

    th,
    td {
        padding: 12px;
        text-align: left;
        border-bottom: 1px solid #ddd;
    }

    th {
        font-weight: 600;
    }

    .url-cell {
        font-family: monospace;
        font-size: 0.9em;
    }

    .loading {
        text-align: center;
        padding: 20px;
        color: #666;
    }

    @media (prefers-color-scheme: dark) {
        th,
        td {
            border-bottom-color: #333;
        }

        .loading {
            color: #999;
        }
    }

    .action-buttons {
        display: flex;
        gap: 8px;
    }

    .icon-button {
        padding: 4px;
        background: none;
        border: none;
        cursor: pointer;
        color: #666;
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease-in-out;
    }

    .icon-button:hover {
        color: #396cd8;
        background-color: rgba(57, 108, 216, 0.1);
    }

    .tooltip-container {
        position: relative;
    }

    .tooltip {
        visibility: hidden;
        position: absolute;
        background-color: #333;
        color: white;
        text-align: center;
        padding: 4px 8px;
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

    .tooltip-container:hover .tooltip {
        visibility: visible;
        opacity: 1;
    }

    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .modal {
        background-color: white;
        padding: 32px;
        border-radius: 12px;
        width: 600px;
        max-width: 90%;
    }

    .modal h2 {
        font-size: 24px;
        margin-bottom: 24px;
        font-weight: 600;
    }

    .form-group {
        margin-bottom: 24px;
    }

    .form-group label {
        display: block;
        margin-bottom: 10px;
        font-weight: 500;
        font-size: 16px;
    }

    .form-group input {
        width: 100%;
        padding: 12px;
        border: 1px solid #ddd;
        border-radius: 6px;
        font-size: 16px;
        line-height: 1.5;
    }

    .modal-buttons {
        display: flex;
        justify-content: flex-end;
        gap: 16px;
        margin-top: 32px;
    }

    .cancel-button,
    .save-button {
        padding: 12px 24px;
        border-radius: 6px;
        cursor: pointer;
        font-weight: 500;
        font-size: 16px;
        min-width: 100px;
    }

    .save-button {
        background-color: #396cd8;
        color: white;
        border: none;
    }

    .save-button:hover {
        background-color: #2857b8;
    }

    .cancel-button {
        background: none;
        border: 1px solid #ddd;
    }

    .cancel-button:hover {
        background-color: #f5f5f5;
    }

    @media (prefers-color-scheme: dark) {
        .modal {
            background-color: #1f1f1f;
            color: #f6f6f6;
        }

        .form-group input {
            background-color: #333;
            border-color: #444;
            color: #f6f6f6;
        }

        .cancel-button {
            border-color: #444;
            color: #f6f6f6;
        }

        .cancel-button:hover {
            background-color: #2a2a2a;
        }

        .save-button:hover {
            background-color: #2857b8;
        }
    }

    .import-button {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 12px 20px;
        background-color: #396cd8;
        color: white;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        font-size: 16px;
        font-weight: 500;
        transition: background-color 0.2s;
    }

    .import-button:hover {
        background-color: #2857b8;
    }

    .import-button svg {
        width: 16px;
        height: 16px;
    }

    .modal h2 {
        font-size: 28px;
        margin-bottom: 32px;
        font-weight: 600;
    }

    .form-group input {
        width: 100%;
        padding: 14px;
        border: 1px solid #ddd;
        border-radius: 8px;
        font-size: 16px;
        line-height: 1.5;
    }

    .form-group input::placeholder {
        color: #999;
    }

    @media (prefers-color-scheme: dark) {
        .import-button {
            background-color: #4a7be0;
        }

        .import-button:hover {
            background-color: #396cd8;
        }

        .form-group input::placeholder {
            color: #666;
        }
    }

    .delete-button:hover {
        color: #dc3545;
        background-color: rgba(220, 53, 69, 0.1);
    }

    @media (prefers-color-scheme: dark) {
        .delete-button:hover {
            color: #ff4d4d;
            background-color: rgba(255, 77, 77, 0.1);
        }
    }

    .confirm-modal {
        width: 400px;
    }

    .confirm-modal p {
        margin-bottom: 24px;
        font-size: 16px;
        line-height: 1.5;
        color: #666;
    }

    .confirm-delete {
        background-color: #dc3545;
        color: white;
        border: none;
    }

    .confirm-delete:hover {
        background-color: #c82333;
    }

    @media (prefers-color-scheme: dark) {
        .confirm-modal p {
            color: #999;
        }

        .confirm-delete {
            background-color: #dc3545;
        }

        .confirm-delete:hover {
            background-color: #bd2130;
        }
    }

    /* Update the confirm modal button styles */
    .confirm-modal .modal-buttons {
        display: flex;
        justify-content: flex-end;
        gap: 16px;
        margin-top: 32px;
    }

    .confirm-modal .cancel-button,
    .confirm-modal .confirm-delete {
        padding: 12px 24px;
        border-radius: 6px;
        cursor: pointer;
        font-weight: 500;
        font-size: 16px;
        min-width: 100px;
        flex: 1; /* Make buttons take equal space */
        max-width: 150px; /* Limit maximum width */
    }

    .confirm-modal .confirm-delete {
        background-color: #dc3545;
        color: white;
        border: none;
    }

    .confirm-modal .confirm-delete:hover {
        background-color: #c82333;
    }

    .confirm-modal .cancel-button {
        background: none;
        border: 1px solid #ddd;
    }

    @media (prefers-color-scheme: dark) {
        .confirm-modal .cancel-button {
            border-color: #444;
            color: #f6f6f6;
        }

        .confirm-modal .confirm-delete:hover {
            background-color: #bd2130;
        }
    }
</style>
