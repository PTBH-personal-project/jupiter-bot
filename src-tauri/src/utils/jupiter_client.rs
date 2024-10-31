use jupiter_swap_api_client::JupiterSwapApiClient;

use super::NetworkConstants;

pub fn setup_jupiter_client() -> JupiterSwapApiClient {
    JupiterSwapApiClient::new(NetworkConstants::PUBLIC_JUPITER_API.to_string())
}
