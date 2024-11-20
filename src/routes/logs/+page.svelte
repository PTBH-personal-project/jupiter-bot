<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { fade } from 'svelte/transition';

    interface StrategyLog {
        id: number;
        strategy_id: number;
        message: string;
        timestamp: string;
        status: string;
    }

    let logs: StrategyLog[] = [];
    let loading = true;
    let selectedStrategyId: number | null = null;
    let selectedLog: StrategyLog | null = null;
    let showDetailsModal = false;

    async function loadLogs() {
        try {
            loading = true;
            const newLogs = await invoke("get_strategy_logs", {
                strategyId: selectedStrategyId,
                limit: 100
            });
            
            // Only update logs if there are changes
            if (JSON.stringify(newLogs) !== JSON.stringify(logs)) {
                logs = newLogs as StrategyLog[];
            }
        } catch (error) {
            console.error("Error loading logs:", error);
        } finally {
            loading = false;
        }
    }

    function showDetails(log: StrategyLog) {
        selectedLog = log;
        showDetailsModal = true;
    }

    function closeModal() {
        showDetailsModal = false;
        selectedLog = null;
    }

    // Truncate message for table display
    function truncateMessage(message: string, length: number = 50): string {
        return message.length > length ? message.substring(0, length) + '...' : message;
    }

    // Refresh logs every 5 seconds
    let interval: ReturnType<typeof setInterval> | null = null;
    onMount(() => {
        loadLogs();
        interval = setInterval(loadLogs, 5000);
        return () => clearInterval(interval as NodeJS.Timeout);
    });

    // Helper function to get status badge classes
    function getStatusBadgeClass(status: string): string {
        return status.toLowerCase() === 'success' ? 'status-badge success' : 'status-badge error';
    }

    // Helper function to get status icon
    function getStatusIcon(status: string): string {
        if (status.toLowerCase() === 'success') {
            return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M20 6L9 17l-5-5"/>
            </svg>`;
        }
        return `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/>
            <line x1="15" y1="9" x2="9" y2="15"/>
            <line x1="9" y1="9" x2="15" y2="15"/>
        </svg>`;
    }
</script>

<div class="container">
    <h1>Strategy Execution Logs</h1>

    {#if loading}
        <div class="loading">Loading logs...</div>
    {:else}
        <table class="logs-table">
            <thead>
                <tr>
                    <th>Time</th>
                    <th>Strategy ID</th>
                    <th>Status</th>
                    <th>Message</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each logs as log (log.id)}
                    <tr 
                        class="log-row"
                        in:fade|local={{ duration: 300 }}
                    >
                        <td>{new Date(log.timestamp).toLocaleString()}</td>
                        <td>{log.strategy_id}</td>
                        <td>
                            <div class={getStatusBadgeClass(log.status)}>
                                <span class="status-icon">
                                    {@html getStatusIcon(log.status)}
                                </span>
                                {log.status}
                            </div>
                        </td>
                        <td>
                            <div class="message-cell" title={log.message}>
                                {truncateMessage(log.message)}
                            </div>
                        </td>
                        <td>
                            <button class="details-button" on:click={() => showDetails(log)}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"/>
                                    <line x1="12" y1="16" x2="12" y2="12"/>
                                    <line x1="12" y1="8" x2="12" y2="8"/>
                                </svg>
                            </button>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}
</div>

<!-- Details Modal -->
{#if showDetailsModal && selectedLog}
    <div class="modal-backdrop" on:click={closeModal}>
        <div class="modal-content" on:click|stopPropagation>
            <div class="modal-header">
                <h2>Log Details</h2>
                <button class="close-button" on:click={closeModal}>×</button>
            </div>
            <div class="modal-body">
                <div class="detail-row">
                    <span class="detail-label">Time:</span>
                    <span>{new Date(selectedLog.timestamp).toLocaleString()}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-label">Strategy ID:</span>
                    <span>{selectedLog.strategy_id}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-label">Status:</span>
                    <div class={getStatusBadgeClass(selectedLog.status)}>
                        <span class="status-icon">
                            {@html getStatusIcon(selectedLog.status)}
                        </span>
                        {selectedLog.status}
                    </div>
                </div>
                <div class="detail-row">
                    <span class="detail-label">Message:</span>
                    <span class="detail-message">{selectedLog.message}</span>
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .container {
        padding: 2rem;
    }

    .logs-table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 2rem;
    }

    .logs-table th,
    .logs-table td {
        padding: 1rem;
        text-align: left;
        border-bottom: 1px solid #eee;
    }

    .log-row.success {
        background-color: rgba(0, 255, 0, 0.05);
    }

    .log-row.error {
        background-color: rgba(255, 0, 0, 0.05);
    }

    .status-badge {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.25rem 0.75rem;
        border-radius: 9999px;
        font-size: 0.875rem;
        font-weight: 500;
        white-space: nowrap;
    }

    .status-badge.success {
        background-color: rgba(34, 197, 94, 0.1);
        color: rgb(34, 197, 94);
    }

    .status-badge.error {
        background-color: rgba(239, 68, 68, 0.1);
        color: rgb(239, 68, 68);
    }

    .status-icon {
        display: inline-flex;
        align-items: center;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .status-badge.success {
            background-color: rgba(34, 197, 94, 0.2);
            color: rgb(74, 222, 128);
        }

        .status-badge.error {
            background-color: rgba(239, 68, 68, 0.2);
            color: rgb(248, 113, 113);
        }

        .logs-table th {
            color: #999;
            border-bottom-color: rgba(255, 255, 255, 0.1);
        }

        .logs-table td {
            border-bottom-color: rgba(255, 255, 255, 0.05);
        }

        .log-row:hover {
            background-color: rgba(255, 255, 255, 0.03);
        }
    }

    /* Add hover effect for rows */
    .log-row {
        transition: background-color 0.2s;
    }

    .log-row:hover {
        background-color: rgba(0, 0, 0, 0.02);
    }

    /* Loading state */
    .loading {
        text-align: center;
        padding: 2rem;
        color: #666;
    }

    .details-button {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.5rem;
        color: #666;
        transition: color 0.2s;
    }

    .details-button:hover {
        color: #000;
    }

    /* Modal styles */
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .modal-content {
        background: white;
        border-radius: 8px;
        width: 95%;
        max-width: 1000px;
        max-height: 90vh;
        overflow-y: auto;
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        border-bottom: 1px solid #eee;
    }

    .modal-body {
        padding: 1rem;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0.5rem;
    }

    .detail-row {
        margin-bottom: 1rem;
        display: flex;
        align-items: flex-start;
        gap: 1rem;
    }

    .detail-label {
        font-weight: 500;
        min-width: 120px;
        color: #666;
    }

    .detail-message {
        white-space: pre-wrap;
        word-break: break-word;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .modal-content {
            background: #1a1a1a;
            color: #fff;
        }

        .modal-header {
            border-bottom-color: #333;
        }

        .close-button {
            color: #fff;
        }

        .details-button {
            color: #999;
        }

        .details-button:hover {
            color: #fff;
        }

        .detail-label {
            color: #999;
        }
    }

    .message-cell {
        position: relative;
        cursor: help;
    }

    .message-cell:hover::after {
        content: attr(title);
        position: absolute;
        left: 0;
        top: 100%;
        background: #333;
        color: white;
        padding: 0.5rem;
        border-radius: 4px;
        font-size: 0.875rem;
        max-width: 400px;
        word-wrap: break-word;
        z-index: 1000;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .message-cell:hover::after {
            background: #1a1a1a;
            border: 1px solid #333;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.4);
        }
    }
</style>