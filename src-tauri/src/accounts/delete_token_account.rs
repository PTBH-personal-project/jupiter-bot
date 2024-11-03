use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_token::instruction::{self as token_instruction, MAX_SIGNERS};
use tauri::{command, State};

use crate::AppState;

use super::get_account_from_public_key;

#[tauri::command]
pub async fn delete_token_account(
    state: State<'_, AppState>,
    owner: String,
    token_account_pubkey: String,
) -> Result<(), String> {
    // Parse the token account pubkey
    let owner_pubkey = Pubkey::from_str(&owner).map_err(|e| e.to_string())?;
    let token_account_pubkey =
        Pubkey::from_str(&token_account_pubkey).map_err(|e| e.to_string())?;

    // Get the RPC client
    let rpc_client = &state.rpc_client;
    let db = &state.db;

    let owner_account = get_account_from_public_key(db, &owner)
        .await
        .map_err(|e| e.to_string())?;
    if owner_account.is_none() {
        return Err("Owner account not found".to_string());
    }

    let owner_keypair = Keypair::from_base58_string(&owner_account.unwrap().private_key);
    let receiver_pubkey = Pubkey::from_str("4bPLCzXqDiDukQuRxkWz6n2Dw47iwDQjkaqngCspSSFF")
        .map_err(|e| format!("Failed to parse receiver pubkey: {}", e))?;

    println!("token_account_pubkey: {:?}", token_account_pubkey);
    let token_account_data = rpc_client
        .get_token_account(&token_account_pubkey)
        .map_err(|e| format!("Failed to get token account: {}", e))?
        .unwrap();

    // Create the close account instruction
    let burn_amount_instruction = token_instruction::burn(
        &spl_token::id(),
        &token_account_pubkey,
        &Pubkey::from_str(token_account_data.mint.as_str())
            .map_err(|e| format!("Failed to parse mint pubkey: {}", e))?,
        &owner_pubkey,
        &[&owner_keypair.pubkey()],
        token_account_data
            .token_amount
            .amount
            .parse::<u64>()
            .unwrap(),
    )
    .map_err(|e| format!("Failed to create burn amount instruction: {}", e))?;
    let close_instruction = token_instruction::close_account(
        &spl_token::id(),
        &token_account_pubkey,
        &receiver_pubkey,
        &owner_pubkey,
        &[&owner_pubkey],
    )
    .map_err(|e| e.to_string())?;

    // Create and send the transaction
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(|e| e.to_string())?;

    let transaction = Transaction::new_signed_with_payer(
        &[burn_amount_instruction, close_instruction],
        Some(&owner_pubkey),
        &[&owner_keypair],
        recent_blockhash,
    );

    println!("Start sending transaction");

    let tx_result = rpc_client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| e.to_string())?;

    println!("tx_result: {:?}", tx_result);

    Ok(())
}
