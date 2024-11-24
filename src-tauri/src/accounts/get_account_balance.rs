use std::str::FromStr;
use spl_associated_token_account::get_associated_token_address;

use crate::AppState;

#[tauri::command]
pub async fn get_account_balance(
    state: tauri::State<'_, AppState>,
    public_key: String,
    token_address: Option<String>,
) -> Result<u64, String> {
    println!("get_account_balance: {:?}", public_key);
    let rpc_client = &state.rpc_client;
    let pubkey = solana_sdk::pubkey::Pubkey::from_str(&public_key)
        .map_err(|e| format!("Invalid public key: {}", e))?;

    match token_address {
        None => {
        let balance = rpc_client
        .get_balance(&pubkey)
        .map_err(|e| format!("Failed to get balance: {}", e))?;
            Ok(balance)
        }
        Some(token_address) => {
            let token_pubkey = solana_sdk::pubkey::Pubkey::from_str(&token_address)
                .map_err(|e| format!("Invalid token address: {}", e))?;
            let associated_token_account = get_associated_token_address(&pubkey, &token_pubkey);
            let balance = rpc_client
                .get_token_account_balance(&associated_token_account);
            match balance {
                Ok(balance) => Ok(balance.amount.parse::<u64>().map_err(|e| format!("Failed to parse balance amount: {}", e))?),
                Err(e) => {
                    if e.to_string().contains("-32602") {
                        // Account not exist
                        return Ok(0);
                    }
                    Err(format!("Failed to get balance: {}", e))
                }
            }
        }
    }
}
