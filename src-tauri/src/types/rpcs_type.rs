use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RpcEndpoint {
    pub id: u16,
    pub name: String,
    pub url: String,
    pub description: String,
}
