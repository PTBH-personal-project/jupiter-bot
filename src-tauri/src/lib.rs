mod accounts;
mod rpcs;
mod tokens;
mod types;
mod utils;

use std::str::FromStr;

pub use accounts::*;
use jupiter_swap_api_client::quote::{QuoteRequest, SwapMode};
pub use rpcs::*;
use solana_sdk::{
    msg,
    pubkey::{self, Pubkey},
};
use tauri::{Manager, State};
use tokens::*;
pub use types::*;
pub use utils::*;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(app: State<'_, AppState>, token_address: &str) -> Result<String, String> {
    let jupiter_client = &app.jupiter_client;
    let input_mint = Pubkey::from_str(token_address).unwrap();
    let output_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let quote_request = QuoteRequest {
        amount: 1000000,
        input_mint,
        output_mint,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };
    let quote_response = jupiter_client.quote(&quote_request).await.unwrap();
    msg!("{:?}", &quote_response);
    Ok(format!(
        "Number of out amount {}",
        &quote_response.out_amount
    ))
}

#[tauri::command]
fn capitalize(s: &str) -> String {
    s.to_uppercase()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            capitalize,
            import_account,
            get_accounts,
            get_account_by_public_key,
            get_all_token_account_for_pubkey,
            delete_account,
            delete_token_account,
            toggle_account_status,
            update_account,
            get_account_balance,
            get_rpcs_endpoint,
            update_rpc,
            import_rpc,
            delete_rpc,
            get_token_info,
            get_token_price,
            import_token,
            get_all_tokens,
            delete_token,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let db = setup_db(&app).await;
    let rpc_client = setup_rpc_client();
    let jupiter_client = setup_jupiter_client();
    app.manage(AppState {
        db,
        rpc_client,
        jupiter_client,
    });
    app.run(|_, _| {});
}
