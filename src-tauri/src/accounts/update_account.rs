use solana_sdk::signer::{keypair::Keypair, Signer};

use crate::AppState;

#[tauri::command]
pub async fn update_account(
    state: tauri::State<'_, AppState>,
    account_id: u16,
    account_name: String,
    description: Option<String>,
    private_key: String,
) -> Result<(), String> {
    let db = &state.db;
    let keypair = Keypair::from_base58_string(&private_key);
    let public_key = keypair.pubkey().to_string();
    sqlx::query("UPDATE accounts SET name = $1, description = $2, private_key = $3, public_key = $4 WHERE id = $5")
        .bind(account_name)
        .bind(description)
        .bind(private_key)
        .bind(public_key)
        .bind(account_id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
