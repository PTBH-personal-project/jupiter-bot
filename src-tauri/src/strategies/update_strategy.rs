use crate::types::*;

#[tauri::command]
pub async fn update_strategy(
    state: tauri::State<'_, AppState>,
    strategy_id: i64,
    interval_time: i64,
    price: i64,
    amount: i64,
    prioritization_fee: i64,
    slippage: i64,
) -> Result<(), String> {
    if slippage > 10000 {
        return Err("Slippage cannot be greater than 100%".to_string());
    }
    let db = &state.db;

    sqlx::query(
        "UPDATE strategies SET 
            interval_time = ?, 
            price = ?, 
            amount = ?, 
            prioritization_fee = ?, 
            slippage = ?
        WHERE id = ?",
    )
    .bind(interval_time)
    .bind(price)
    .bind(amount)
    .bind(prioritization_fee)
    .bind(slippage)
    .bind(strategy_id)
    .execute(db)
    .await
    .map_err(|e| format!("Failed to update strategy: {}", e))?;

    Ok(())
}
