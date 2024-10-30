use solana_client::rpc_client::RpcClient;

use super::NetworkConstants;

pub fn setup_rpc_client() -> RpcClient {
    RpcClient::new(NetworkConstants::PUBLIC_RPC_URL.to_string())
}
