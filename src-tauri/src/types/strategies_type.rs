use std::{fmt::format, str::FromStr};

use jupiter_swap_api_client::{
    quote::{QuoteRequest, QuoteResponse},
    swap::SwapRequest,
    transaction_config::{PrioritizationFeeLamports, TransactionConfig},
    JupiterSwapApiClient,
};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::VersionedTransaction
};
use base64;

use crate::{format_amount, AdddressConstants};

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

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
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
            StrategyType::Buy => format!(
                "Buy {} SOL for tokens {} at price {}",
                format_amount(self.amount, 9),
                self.token_name,
                format_amount(self.price, 9)
            ),
            StrategyType::Sell => format!(
                "Sell {} {} for SOL at price {}",
                format_amount(self.amount, self.decimals as u8),
                self.token_name,
                self.price
            ),
        }
    }

    pub fn get_keypair(&self) -> Keypair {
        Keypair::from_base58_string(&self.account_private_key)
    }

    pub async fn execute(
        &self,
        jupiter_client: &JupiterSwapApiClient,
        rpc_client: &RpcClient,
    ) -> StrategyExecutionResult {
        match self.strategy_type {
            StrategyType::Buy => self.execute_buy(jupiter_client, rpc_client).await,
            StrategyType::Sell => self.execute_sell(jupiter_client, rpc_client).await,
        }
    }

    fn to_stategy_execution_with_error(
        &self,
        status: StrategyExecutionStatus,
        message: String,
    ) -> StrategyExecutionResult {
        StrategyExecutionResult {
            tx_hash: None,
            status,
            message,
            strategy_id: self.id,
            strategy_overview: self.get_strategy_overview(),
            current_time: format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")),
        }
    }

    async fn execute_buy(
        &self,
        jupiter_client: &JupiterSwapApiClient,
        rpc_client: &RpcClient,
    ) -> StrategyExecutionResult {
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
        if quote_response.is_err() {
            return self.to_stategy_execution_with_error(
                StrategyExecutionStatus::Failed,
                format!(
                    "Get the quote response failed: {}",
                    quote_response.unwrap_err()
                ),
            );
        }
        let quote_response = quote_response.unwrap();
        let price =
            (self.amount as u64) * (10u64.pow(self.decimals as u32)) / quote_response.out_amount;
        if price > self.price as u64 {
            return self.to_stategy_execution_with_error(
                StrategyExecutionStatus::MissingPrice,
                format!(
                    "The price is higher than the price set: {} > {}",
                    price, self.price
                ),
            );
        }
        let keypair = self.get_keypair();

        let swap_transaction = jupiter_client
            .swap(&SwapRequest {
                user_public_key: keypair.pubkey(),
                quote_response: quote_response,
                config: TransactionConfig::default(),
            })
            .await;

        if swap_transaction.is_err() {
            return self.to_stategy_execution_with_error(
                StrategyExecutionStatus::Failed,
                "Get the swap instruction failed".to_string(),
            );
        }
        let data = swap_transaction.unwrap().swap_transaction;
        let transaction = bincode::deserialize::<VersionedTransaction>(&data)
            .map_err(|e| format!("Failed to deserialize transaction: {}", e))
            .unwrap();

        let signed_transaction =
            VersionedTransaction::try_new(transaction.message, &[&keypair]).unwrap();
        let transaction = signed_transaction;

        let signature = rpc_client.send_and_confirm_transaction(&transaction);

        match signature {
            Ok(sig) => StrategyExecutionResult {
                tx_hash: Some(sig.to_string()),
                status: StrategyExecutionStatus::Success,
                message: "Transaction executed successfully".to_string(),
                strategy_id: self.id,
                strategy_overview: self.get_strategy_overview(),
                current_time: format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")),
            },
            Err(e) => {
                println!("transaction error: {:?}", e.get_transaction_error());
                self.to_stategy_execution_with_error(
                StrategyExecutionStatus::Failed,
                format!("Transaction failed: {}", e),
                )
            }
        }
    }

    async fn execute_sell(
        &self,
        jupiter_client: &JupiterSwapApiClient,
        rpc_client: &RpcClient,
    ) -> StrategyExecutionResult {
        let input_mint = Pubkey::from_str(&self.token_address).unwrap();
        let output_mint = Pubkey::from_str(AdddressConstants::WSOL_ADDRESS).unwrap();
        let mut request = QuoteRequest::default();
        // request.max_accounts = Some(20);
        let quote_request = QuoteRequest {
            amount: self.amount as u64,
            input_mint,
            output_mint,
            slippage_bps: self.slippage as u16,
            ..request
            // ..QuoteRequest::default()
        };

        let quote_response = jupiter_client.quote(&quote_request).await;

        println!("quote_response: {:?}", quote_response);
        if quote_response.is_err() {
            return self.to_stategy_execution_with_error(
                StrategyExecutionStatus::Failed,
                format!("Get the quote response failed: {}", quote_response.unwrap_err()),
            );
        }
        let quote_response = quote_response.unwrap();
        let price = (quote_response.out_amount as u64) * (10u64.pow(self.decimals as u32))
            / self.amount as u64;
        
        println!("price calculated: {}", price);
        if price < self.price as u64 {
            return self.to_stategy_execution_with_error(
                StrategyExecutionStatus::MissingPrice,
                format!(
                    "The price is lower than the price set: {} < {}",
                    price, self.price
                ),
            );
        }
        let keypair = self.get_keypair();

        let mut config = TransactionConfig::default();
        // config.prioritization_fee_lamports = Some(PrioritizationFeeLamports::JitoTipLamports(5000000u64));
        config.prioritization_fee_lamports = Some(PrioritizationFeeLamports::Auto);
        config.dynamic_compute_unit_limit = true;

        let swap_transaction = jupiter_client
            .swap(&SwapRequest {
                user_public_key: keypair.pubkey(),
                quote_response: quote_response,
                config,
            })
            .await;

        if swap_transaction.is_err() {
            return self.to_stategy_execution_with_error(
                StrategyExecutionStatus::Failed,
                "Get the swap instruction failed".to_string(),
            );
        }
        let data = swap_transaction.unwrap().swap_transaction;
        let transaction = bincode::deserialize::<VersionedTransaction>(&data)
            .map_err(|e| format!("Failed to deserialize transaction: {}", e))
            .unwrap();
            
        let signed_transaction =
            VersionedTransaction::try_new(transaction.message, &[&keypair]).unwrap();

        let transaction = signed_transaction;

        let serialized = bincode::serialize(&transaction).unwrap();
        println!("Transaction (base64): {}", base64::encode(&serialized));

        let signature = rpc_client.send_and_confirm_transaction(&transaction);

        match signature {
            Ok(sig) => StrategyExecutionResult {
                tx_hash: Some(sig.to_string()),
                status: StrategyExecutionStatus::Success,
                message: "Transaction executed successfully".to_string(),
                strategy_id: self.id,
                strategy_overview: self.get_strategy_overview(),
                current_time: format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")),
            },
            Err(e) => {
                println!("transaction error: {:?}", e.get_transaction_error());
                self.to_stategy_execution_with_error(
                StrategyExecutionStatus::Failed,
                format!("Transaction failed: {}", e),
                )
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StrategyLog {
    pub id: i64,
    pub strategy_id: i64,
    pub message: String,
    pub timestamp: String,
    pub status: StrategyExecutionStatus,
    pub tx_hash: Option<String>,
}