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
                logo_uri: "".to_string(),
            })
        }
        Err(_) => {
            msg!("Metadata account not found, trying RPL");
            match get_token_metadata_from_rpl(token_address).await {
                Ok(rpl_metadata) => Ok(TokenInfo {
                    name: rpl_metadata.name,
                    address: pubkey.to_string(),
                    symbol: rpl_metadata.symbol,
                    decimals: mint.decimals,
                    total_supply: mint.supply,
                    uri: "".to_string(),
                    logo_uri: rpl_metadata.logo_uri,
                }),
                Err(_) => Ok(TokenInfo {
                    name: "".to_string(),
                    address: pubkey.to_string(),
                    symbol: "".to_string(),
                    decimals: 0,
                    total_supply: 0,
                    uri: "".to_string(),
                    logo_uri: "".to_string(),
                }),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct RplMetadata {
    pub name: String,
    pub symbol: String,
    pub logo_uri: String,
}

pub async fn get_token_metadata_from_rpl(token_address: &str) -> Result<RplMetadata, String> {
    let url = format!("https://raw.githubusercontent.com/solana-labs/token-list/refs/heads/main/src/tokens/solana.tokenlist.json");

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch token list: {}", e))?;

    let token_list: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token list JSON: {}", e))?;

    let tokens = token_list["tokens"]
        .as_array()
        .ok_or("Invalid token list format")?;

    let token_info = tokens
        .iter()
        .find(|token| token["address"].as_str() == Some(token_address))
        .ok_or("Token not found in RPL list")?;

    let name = token_info["name"]
        .as_str()
        .ok_or("Invalid name format")?
        .to_string();

    let symbol = token_info["symbol"]
        .as_str()
        .ok_or("Invalid symbol")?
        .to_string();

    let logo_uri = token_info["logoURI"]
        .as_str()
        .ok_or("Invalid logoURI")?
        .to_string();
    Ok(RplMetadata {
        name,
        symbol,
        logo_uri,
    })
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
                logo_uri: "".to_string(),
            })
        }
        Err(_) => Ok(TokenInfo {
            name: "Base".to_string(),
            address: pubkey.to_string(),
            symbol: "Base".to_string(),
            decimals: 0,
            total_supply: 0,
            uri: "Base".to_string(),
            logo_uri: "".to_string(),
        }),
    }
}
