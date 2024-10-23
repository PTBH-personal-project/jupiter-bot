<script lang="ts">
    let showPopup = false;
    let privateKeyOrPath = "";
    let publicKey = "";

    function togglePopup() {
        showPopup = !showPopup;
    }

    function previewPublicKey() {
        publicKey = privateKeyOrPath.toUpperCase();
    }

    function handleSubmit() {
        console.log("Imported text:", privateKeyOrPath);
        // Here you would typically process the imported text
        // For now, we'll just log it to the console
        showPopup = false;
        privateKeyOrPath = "";
    }
</script>

<div class="accounts-container">
    <h1>Accounts</h1>

    <button on:click={togglePopup}>Import</button>
</div>

{#if showPopup}
    <div class="popup-overlay">
        <div class="popup-content">
            <h2>Import Account</h2>
            <form on:submit|preventDefault={handleSubmit}>
                <textarea
                    bind:value={privateKeyOrPath}
                    placeholder="Paste your private key or path to private key here"
                    rows="2"
                    on:input={previewPublicKey}
                ></textarea>
                <div class="readonly-textarea">
                    {`Public key: ${publicKey}`}
                </div>
                <div class="button-group">
                    <button type="button" on:click={togglePopup}>Cancel</button>
                    <button type="submit">Submit</button>
                </div>
            </form>
        </div>
    </div>
{/if}

<style>
    .accounts-container {
        padding: 20px;
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
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    h2 {
        margin-top: 0;
    }

    textarea {
        width: 100%;
        padding: 10px;
        margin-bottom: 1px;
        border-radius: 8px;
        border: 1px solid #ccc;
        font-family: inherit;
        font-size: 1em;
        box-sizing: border-box;
    }
    .readonly-textarea {
        width: 100%;
        padding: 1px;
        margin-bottom: 1px;
        font-size: 0.875em; /* smaller than normal */
        color: #e0e0e0;
    }

    .button-group {
        display: flex;
        justify-content: flex-end;
        gap: 10px;
    }

    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #0f0f0f;
        background-color: #ffffff;
        transition:
            border-color 0.25s,
            background-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
        cursor: pointer;
    }

    button:hover {
        border-color: #396cd8;
    }

    button:active {
        border-color: #396cd8;
        background-color: #e8e8e8;
    }

    @media (prefers-color-scheme: dark) {
        .popup-content {
            background-color: #2f2f2f;
            color: #f6f6f6;
        }

        textarea {
            background-color: #1f1f1f;
            color: #f6f6f6;
            border-color: #444;
        }

        button {
            color: #ffffff;
            background-color: #0f0f0f98;
        }
        button:active {
            background-color: #0f0f0f69;
        }
    }
</style>
