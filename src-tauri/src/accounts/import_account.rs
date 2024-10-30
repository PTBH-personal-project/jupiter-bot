use crate::types::*;
use solana_sdk::signer::{keypair::Keypair, Signer};

#[tauri::command]
pub async fn import_account(
    state: tauri::State<'_, AppState>,
    account_name: String,
    private_key: String,
    description: Option<String>,
) -> Result<(), String> {
    let db = &state.db;

    let keypair = Keypair::from_base58_string(&private_key);
    let public_key = keypair.pubkey().to_string();

    sqlx::query(
        "INSERT INTO accounts (name, private_key, public_key, description, status) 
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(account_name)
    .bind(private_key)
    .bind(public_key)
    .bind(description.unwrap_or_default())
    .bind("Enabled")
    .execute(db)
    .await
    .map_err(|e| format!("Failed to import account: {}", e))?;

    Ok(())
}
