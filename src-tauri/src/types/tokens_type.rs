use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub uri: String,
    pub logo_uri: String,
}
