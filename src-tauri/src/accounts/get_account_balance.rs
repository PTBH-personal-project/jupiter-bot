use std::str::FromStr;

use solana_client::rpc_client;

#[tauri::command]
pub async fn get_account_balance(public_key: String, token_address: Option<String>) -> Result<u64, String> {
    let cluster_url = "https://api.mainnet-beta.solana.com";
    let rpc_client = rpc_client::RpcClient::new(cluster_url.to_string());
    let pubkey = solana_sdk::pubkey::Pubkey::from_str(&public_key)
        .map_err(|e| format!("Invalid public key: {}", e))?;

    let balance = rpc_client
        .get_balance(&pubkey)
        .map_err(|e| format!("Failed to get balance: {}", e))?;
    Ok(balance)
}