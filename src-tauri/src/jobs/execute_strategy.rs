use jupiter_swap_api_client::JupiterSwapApiClient;

use crate::types::*;
pub async fn check_strategies(
    db: &Db,
    jupiter_client: &JupiterSwapApiClient,
) -> Result<(), String> {
    // Query all active strategies
    let strategy = sqlx::query_as::<_, StrategyWithFullInformation>(
        "SELECT s.*, t.logo_uri, t.decimals, t.name as token_name, a.name as account_name 
         FROM strategies s
         JOIN tokens t ON s.token_address = t.address 
         JOIN accounts a ON s.account_private_key = a.private_key
         WHERE s.status = 'Executing' 
         ORDER BY s.next_time_execute ASC 
         LIMIT 1",
    )
    .fetch_one(db)
    .await;

    match strategy {
        Ok(strategy) => {
            strategy.execute(jupiter_client).await?;
        }
        Err(e) => {
            eprintln!("Error fetching strategy: {}", e);
        }
    }
    Ok(())
}
