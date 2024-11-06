use std::str::FromStr;

use jupiter_swap_api_client::{quote::QuoteRequest, JupiterSwapApiClient};
use solana_sdk::{msg, pubkey::Pubkey};
use tauri::State;

use crate::{AdddressConstants, AppState};

#[tauri::command]
pub async fn get_token_price(
    app: State<'_, AppState>,
    token_address: &str,
    token_decimals: u8,
) -> Result<u64, String> {
    let jupiter_client = &app.jupiter_client;
    let input_mint = Pubkey::from_str(token_address).unwrap();
    let output_mint = Pubkey::from_str(AdddressConstants::USDT_ADDRESS).unwrap();
    if input_mint == output_mint {
        return Ok(1 * 10u64.pow(token_decimals as u32));
    }
    let quote_request = QuoteRequest {
        amount: 1 * 10u64.pow(token_decimals as u32),
        input_mint,
        output_mint,
        slippage_bps: 0,
        ..QuoteRequest::default()
    };
    match jupiter_client.quote(&quote_request).await {
        Ok(quote_response) => Ok(quote_response.out_amount),
        Err(e) => {
            msg!("Error getting token price: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn get_token_price_in_sol(
    app: State<'_, AppState>,
    token_address: &str,
    token_decimals: u8,
) -> Result<u64, String> {
    let jupiter_client = &app.jupiter_client;
    let input_mint = Pubkey::from_str(token_address).unwrap();
    let output_mint = Pubkey::from_str(AdddressConstants::WSOL_ADDRESS).unwrap();
    if input_mint == output_mint {
        return Ok(1 * 10u64.pow(token_decimals as u32));
    }
    let quote_request = QuoteRequest {
        amount: 1 * 10u64.pow(token_decimals as u32),
        input_mint,
        output_mint,
        slippage_bps: 0,
        ..QuoteRequest::default()
    };
    match jupiter_client.quote(&quote_request).await {
        Ok(quote_response) => Ok(quote_response.out_amount),
        Err(e) => {
            msg!("Error getting token price: {}", e);
            Err(e.to_string())
        }
    }
}
