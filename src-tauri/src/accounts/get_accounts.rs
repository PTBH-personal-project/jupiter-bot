use std::str::FromStr;

use crate::types::*;
use solana_account_decoder::UiAccountData;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::pubkey::Pubkey;
use spl_token::ID as TOKEN_PROGRAM_ID;

#[tauri::command]
pub async fn get_accounts(state: tauri::State<'_, AppState>) -> Result<Vec<Account>, String> {
    let db = &state.db;
    let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get accounts: {}", e))?;

    Ok(accounts)
}

#[tauri::command]
pub async fn get_account_by_public_key(
    state: tauri::State<'_, AppState>,
    public_key: String,
) -> Result<Account, String> {
    let db = &state.db;

    let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE public_key = ?")
        .bind(public_key)
        .fetch_optional(db)
        .await
        .map_err(|e| format!("Failed to fetch account: {}", e))?
        .ok_or_else(|| "Account not found".to_string())?;

    Ok(account)
}

#[tauri::command]
pub async fn get_all_token_account_for_pubkey(
    state: tauri::State<'_, AppState>,
    pubkey: String,
) -> Result<Vec<TokenAccount>, String> {
    let rpc_client = &state.rpc_client;
    let keyed_token_accounts = rpc_client
        .get_token_accounts_by_owner(
            &Pubkey::from_str(&pubkey).unwrap(),
            TokenAccountsFilter::ProgramId(TOKEN_PROGRAM_ID),
        )
        .unwrap();
    let mut token_accounts = Vec::new();
    for account in keyed_token_accounts.iter() {
        let token_account = match &account.account.data {
            UiAccountData::Json(parsed) => {
                let parsed_info = parsed.parsed.get("info").unwrap();
                parsed
                    .parsed
                    .get("info")
                    .unwrap()
                    .get("tokenAmount")
                    .unwrap()
                    .to_string();
                let mint = parsed_info
                    .get("mint")
                    .unwrap()
                    .to_string()
                    .trim_matches('"')
                    .to_string();
                let amount = parsed_info
                    .get("tokenAmount")
                    .unwrap()
                    .get("uiAmountString")
                    .unwrap()
                    .to_string()
                    .trim_matches('"')
                    .to_string();
                TokenAccount {
                    pubkey: account.pubkey.to_string(),
                    mint,
                    amount,
                }
            }
            _ => TokenAccount {
                pubkey: account.pubkey.to_string(),
                mint: Pubkey::default().to_string(),
                amount: "0".to_string(),
            },
        };
        token_accounts.push(token_account);
    }

    Ok(token_accounts)
}
