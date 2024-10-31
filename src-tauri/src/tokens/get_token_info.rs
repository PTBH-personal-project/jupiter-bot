use std::str::FromStr;

use mpl_token_metadata::accounts::Metadata;
use solana_sdk::{msg, program_pack::Pack, pubkey::Pubkey};
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
    let mint_data = rpc_client.get_account(&pubkey).unwrap();
    let mint = Mint::unpack(&mint_data.data).unwrap();

    let metadata_address = Metadata::find_pda(&pubkey).0;
    let metadata_account = rpc_client.get_account(&metadata_address).unwrap();
    let metadata_data = Metadata::from_bytes(&metadata_account.data).unwrap();

    Ok(TokenInfo {
        name: metadata_data.name.trim().to_string(),
        address: token_address,
        symbol: metadata_data.symbol.trim().to_string(),
        decimals: mint.decimals,
        total_supply: mint.supply,
        uri: metadata_data.uri.trim().to_string(),
    })
}
