use std::{fmt::format, str::FromStr};

use jupiter_swap_api_client::{quote::{QuoteRequest, QuoteResponse}, JupiterSwapApiClient};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use tauri::State;

use crate::{format_amount, AdddressConstants};

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
    pub strategy_type: StrategyType,
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
    pub strategy_type: StrategyType,
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
    pub strategy_overview: String,
    pub strategy_id: i64,
    pub current_time: String,
}

impl StrategyWithFullInformation {
    pub const POW_9: f64 = 1_000_000_000.0;
    pub fn get_strategy_overview(&self) -> String {
        match self.strategy_type {
            StrategyType::Buy => format!("Buy {} SOL for tokens {} at price {}", format_amount(self.amount, 9), self.token_name, format_amount(self.price, 9)),
            StrategyType::Sell => format!("Sell {} {} for SOL at price {}", format_amount(self.amount, self.decimals as u8), self.token_name, self.price),
        }
    }
    pub async fn execute(&self, jupiter_client: &JupiterSwapApiClient) -> StrategyExecutionResult {
        let current_price = get_token_price_in_sol_internal(
            jupiter_client,
            &self.token_address,
            self.decimals as u8,
        )
        .await;
        let current_time = format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
        if current_price.is_err() {
            return StrategyExecutionResult {
                tx_hash: None,
                status: StrategyExecutionStatus::Failed,
                message: "Get the price of the token failed".to_string(),
                strategy_id: self.id,
                strategy_overview: self.get_strategy_overview(),
                current_time,
            };
        }
        let current_price = current_price.unwrap() as i64;
        match self.strategy_type {
            StrategyType::Buy => {
                if current_price > self.price {
                    return StrategyExecutionResult {
                        tx_hash: None,
                        status: StrategyExecutionStatus::MissingPrice,
                        message: format!("The price is higher than the price set: {} > {}", current_price, self.price),
                        strategy_id: self.id,
                        strategy_overview: self.get_strategy_overview(),
                        current_time,
                    };
                } else {
                    return StrategyExecutionResult {
                        tx_hash: None,
                        status: StrategyExecutionStatus::Success,
                        message: "Success".to_string(),
                        strategy_id: self.id,
                        strategy_overview: self.get_strategy_overview(),
                        current_time,
                    };
                }
            }
            StrategyType::Sell => {
                if current_price < self.price {
                    return StrategyExecutionResult {
                        tx_hash: None,
                        status: StrategyExecutionStatus::MissingPrice,
                        message: format!("The price is lower than the price set: {} < {}", current_price, self.price),
                        strategy_id: self.id,
                        strategy_overview: self.get_strategy_overview(),
                        current_time,
                    };
                } else {
                    return StrategyExecutionResult {
                        tx_hash: None,
                        status: StrategyExecutionStatus::Success,
                        message: "Success".to_string(),
                        strategy_id: self.id,
                        strategy_overview: self.get_strategy_overview(),
                        current_time,
                    };
                }
            }
        }
    }

    async fn execute_buy(&self, jupiter_client: &JupiterSwapApiClient, current_time: String) -> StrategyExecutionResult {
        let input_mint = Pubkey::from_str(AdddressConstants::WSOL_ADDRESS).unwrap();
        let output_mint = Pubkey::from_str(&self.token_address).unwrap();
        let quote_request = QuoteRequest {
            amount: self.amount as u64,
            input_mint,
            output_mint,
            slippage_bps: self.slippage as u16,
            ..QuoteRequest::default()
        };
        let quote_response = jupiter_client.quote(&quote_request).await;
        StrategyExecutionResult {
            tx_hash: None,
            status: StrategyExecutionStatus::Success,
            message: "Success".to_string(),
            strategy_id: self.id,
            strategy_overview: self.get_strategy_overview(),
            current_time: current_time,
        }
    }
}

async fn get_token_price_in_sol_internal(
    jupiter_client: &JupiterSwapApiClient,
    token_address: &str,
    token_decimals: u8,
) -> Result<u64, String> {
    let input_mint = Pubkey::from_str(token_address).unwrap();
    let output_mint = Pubkey::from_str(AdddressConstants::WSOL_ADDRESS).unwrap();
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
            println!("Error getting token price: {}", e);
            Err(e.to_string())
        }
    }
}