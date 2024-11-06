use std::str::FromStr;

use jupiter_swap_api_client::JupiterSwapApiClient;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use tauri::State;

use crate::get_token_price_in_sol_internal;

use super::AppState;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum StrategyStatus {
    Executing,
    Executed,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum StrategyType {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Strategy {
    pub id: i64,
    pub strategy_type: String,
    pub status: StrategyStatus,
    pub next_time_execute: i64,
    pub interval_time: i64,
    pub account_private_key: String,
    pub token_address: String,
    pub price: i64,
    pub amount: i64,
    pub prioritization_fee: i64,
    pub slippage: i64,
    pub tx_hash: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct StrategyWithFullInformation {
    pub id: i64,
    pub strategy_type: String,
    pub status: StrategyStatus,
    pub next_time_execute: i64,
    pub interval_time: i64,
    pub account_private_key: String,
    pub token_address: String,
    pub price: i64,
    pub amount: i64,
    pub prioritization_fee: i64,
    pub slippage: i64,
    pub tx_hash: Option<String>,
    pub created_at: Option<String>,
    pub logo_uri: String,
    pub decimals: i64,
    pub token_name: String,
    pub account_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StrategyExecutionStatus {
    Success,
    MissingPrice,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrategyExecutionResult {
    pub tx_hash: Option<String>,
    pub status: StrategyExecutionStatus,
    pub message: String,
}

impl StrategyWithFullInformation {
    pub async fn execute(&self, jupiter_client: &JupiterSwapApiClient) -> Result<String, String> {
        let current_price = get_token_price_in_sol_internal(
            jupiter_client,
            &self.token_address,
            self.decimals as u8,
        )
        .await?;
        println!(
            "Current price: {} at {}",
            current_price,
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );

        Ok("Success".to_string())
    }
}
