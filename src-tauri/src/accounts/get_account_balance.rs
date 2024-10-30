use std::str::FromStr;

use crate::AppState;

#[tauri::command]
pub async fn get_account_balance(
    state: tauri::State<'_, AppState>,
    public_key: String,
    token_address: Option<String>,
) -> Result<u64, String> {
    let rpc_client = &state.rpc_client;
    let pubkey = solana_sdk::pubkey::Pubkey::from_str(&public_key)
        .map_err(|e| format!("Invalid public key: {}", e))?;

    let balance = rpc_client
        .get_balance(&pubkey)
        .map_err(|e| format!("Failed to get balance: {}", e))?;
    Ok(balance)
}
