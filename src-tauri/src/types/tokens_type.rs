use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub decimals: u8,
    pub total_supply: String,
    pub uri: String,
    pub logo_uri: String,
}


// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// #[serde(rename_all = "camelCase")]
// pub struct TokenInfoDb {
//     pub name: String,
//     pub symbol: String,
//     pub address: String,
//     pub decimals: u8,
//     pub uri: String,
//     pub logo_uri: String,
// }
