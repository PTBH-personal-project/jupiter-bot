use std::str::FromStr;

use mpl_token_metadata::accounts::Metadata;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{account::Account, msg, program_pack::Pack, pubkey::Pubkey};
use spl_token::state::Mint;
use tauri::State;

use crate::{types::TokenInfo, AppState};

#[tauri::command]
pub async fn get_token_info(
    state: State<'_, AppState>,
    token_address: String,
) -> Result<TokenInfo, String> {
    let pubkey = Pubkey::from_str(&token_address).unwrap();
    let rpc_client = &state.rpc_client;
    let mint_data = rpc_client
        .get_account(&pubkey)
        .map_err(|_| format!("Failed to get account {}", token_address))?;
    msg!("Mint owner {}", mint_data.owner);
    match mint_data.owner {
        spl_token::ID => get_spl_token_info(rpc_client, &token_address, &mint_data).await,
        spl_token_2022::ID => Err("Token 2022 is not supported".to_string()),
        _ => Err("Not a SPL token".to_string()),
    }
}

pub async fn get_spl_token_info(
    rpc_client: &RpcClient,
    token_address: &str,
    mint_data: &Account,
) -> Result<TokenInfo, String> {
    let pubkey = Pubkey::from_str(token_address).unwrap();

    let mint = Mint::unpack(&mint_data.data).unwrap();
    let metadata_address = Metadata::find_pda(&pubkey).0;
    match rpc_client.get_account(&metadata_address) {
        Ok(metadata_account) => {
            let metadata_data = Metadata::from_bytes(&metadata_account.data).unwrap();
            Ok(TokenInfo {
                name: metadata_data.name.trim().to_string(),
                address: pubkey.to_string(),
                symbol: metadata_data.symbol.trim().to_string(),
                decimals: mint.decimals,
                total_supply: mint.supply,
                uri: metadata_data.uri.trim().to_string(),
            })
        }
        Err(_) => Err("Metadata account not found".to_string()),
    }
}

pub async fn get_spl_token_2022_info(
    rpc_client: &RpcClient,
    token_address: &str,
    mint_data: &Account,
) -> Result<TokenInfo, String> {
    let pubkey = Pubkey::from_str(token_address).unwrap();

    let mint =
        spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(
            &mint_data.data,
        )
        .unwrap();
    msg!("Token is SPL 2022, and mint data is {:?}", mint);
    let metadata_address = Metadata::find_pda(&pubkey).0;
    match rpc_client.get_account(&metadata_address) {
        Ok(metadata_account) => {
            let metadata_data = Metadata::from_bytes(&metadata_account.data).unwrap();
            Ok(TokenInfo {
                name: metadata_data.name.trim().to_string(),
                address: pubkey.to_string(),
                symbol: metadata_data.symbol.trim().to_string(),
                decimals: mint.base.decimals,
                total_supply: mint.base.supply,
                uri: metadata_data.uri.trim().to_string(),
            })
        }
        Err(_) => Ok(TokenInfo {
            name: "Base".to_string(),
            address: pubkey.to_string(),
            symbol: "Base".to_string(),
            decimals: 0,
            total_supply: 0,
            uri: "Base".to_string(),
        }),
    }
}
