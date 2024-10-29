use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum AccountStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: u16,
    pub name: String,
    pub private_key: String,
    pub public_key: String,
    pub description: String,
    pub status: AccountStatus,
}
