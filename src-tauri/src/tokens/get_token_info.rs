use std::str::FromStr;

use solana_sdk::{program_pack::Pack, pubkey::Pubkey};
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

    Ok(TokenInfo {
        address: token_address,
        symbol: "Symbol".to_string(),
        decimals: mint.decimals,
        total_supply: mint.supply,
    })
}
