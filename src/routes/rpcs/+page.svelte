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

    onMount(() => {
        loadRpcEndpoints();
    });
</script>

<div class="container">
    <h1>RPC Endpoints</h1>

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
</style>
