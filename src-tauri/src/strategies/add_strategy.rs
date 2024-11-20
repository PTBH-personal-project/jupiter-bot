use crate::types::*;

#[tauri::command]
pub async fn add_strategy(
    state: tauri::State<'_, AppState>,
    strategy_type: String,
    interval_time: i64,
    account_private_key: String,
    token_address: String,
    price: i64,
    amount: i64,
    prioritization_fee: i64,
    slippage: i64,
) -> Result<(), String> {
    if slippage > 10000 {
        return Err("Slippage cannot be greater than 100%".to_string());
    }
    let db = &state.db;

    // Calculate next_time_execute as current time + interval_time
    let next_time_execute = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    sqlx::query(
        "INSERT INTO strategies (
            strategy_type, 
            status, 
            next_time_execute, 
            interval_time, 
            account_private_key, 
            token_address, 
            price, 
            amount, 
            prioritization_fee, 
            slippage
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(strategy_type)
    .bind("Executing") // Default status
    .bind(next_time_execute)
    .bind(interval_time)
    .bind(account_private_key)
    .bind(token_address)
    .bind(price)
    .bind(amount)
    .bind(prioritization_fee)
    .bind(slippage)
    .execute(db)
    .await
    .map_err(|e| format!("Failed to add strategy: {}", e))?;

    Ok(())
}
