use jupiter_swap_api_client::JupiterSwapApiClient;
use solana_client::rpc_client::RpcClient;

use crate::types::*;
pub async fn check_strategies(
    db: &Db,
    jupiter_client: &JupiterSwapApiClient,
    rpc_client: &RpcClient,
) -> Result<(), String> {
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
            let execute_result = strategy.execute(jupiter_client, rpc_client).await;
            // Insert log into database
            sqlx::query(
                "INSERT INTO strategy_logs (strategy_id, message, status) VALUES (?, ?, ?)"
            )
            .bind(strategy.id)
            .bind(execute_result.message)
            .bind(execute_result.status)
            .execute(db)
            .await
            .map_err(|e| format!("Failed to log strategy execution: {}", e))?;
        }
        Err(e) => {
            eprintln!("Error fetching strategy: {}", e);
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn get_strategy_logs(
    state: tauri::State<'_, AppState>,
    strategy_id: Option<i64>,
    limit: Option<i64>,
) -> Result<Vec<StrategyLog>, String> {
    let db = &state.db;
    let limit = limit.unwrap_or(100);

    let query = match strategy_id {
        Some(id) => sqlx::query_as::<_, StrategyLog>(
            "SELECT * FROM strategy_logs WHERE strategy_id = ? ORDER BY timestamp DESC LIMIT ?"
        )
        .bind(id)
        .bind(limit),
        None => sqlx::query_as::<_, StrategyLog>(
            "SELECT * FROM strategy_logs ORDER BY timestamp DESC LIMIT ?"
        )
        .bind(limit),
    };

    query
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to fetch logs: {}", e))
}
