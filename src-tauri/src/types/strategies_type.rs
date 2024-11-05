use serde::{Deserialize, Serialize};

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
