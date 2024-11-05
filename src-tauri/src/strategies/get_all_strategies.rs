use crate::types::*;

#[tauri::command]
pub async fn get_all_strategies(state: tauri::State<'_, AppState>) -> Result<Vec<Strategy>, String> {
    let db = &state.db;

    sqlx::query_as::<_, Strategy>(
        "SELECT 
            id,
            strategy_type,
            status,
            next_time_execute,
            interval_time,
            account_private_key,
            token_address,
            price,
            amount,
            prioritization_fee,
            slippage,
            tx_hash,
            created_at
        FROM strategies"
    )
    .fetch_all(db)
    .await
    .map_err(|e| format!("Failed to get strategies: {}", e))
}

#[tauri::command]
pub async fn get_all_strategies_with_full_information(state: tauri::State<'_, AppState>) -> Result<Vec<StrategyWithFullInformation>, String> {
    let db = &state.db;

    sqlx::query_as::<_, StrategyWithFullInformation>(
        "SELECT 
            s.id,
            s.strategy_type,
            s.status,
            s.next_time_execute,
            s.interval_time,
            s.account_private_key,
            s.token_address,
            s.price,
            s.amount,
            s.prioritization_fee,
            s.slippage,
            s.tx_hash,
            s.created_at,
            t.logo_uri,
            t.decimals,
            t.name as token_name,
            a.name as account_name
        FROM strategies s
        LEFT JOIN tokens t ON s.token_address = t.address
        LEFT JOIN accounts a ON s.account_private_key = a.private_key"
    )
    .fetch_all(db)
    .await
    .map_err(|e| format!("Failed to get strategies: {}", e))
}