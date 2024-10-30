use solana_client::rpc_client::RpcClient;
use sqlx::{Pool, Sqlite};

pub type Db = Pool<Sqlite>;
pub struct AppState {
    pub db: Db,
    pub rpc_client: RpcClient,
}
