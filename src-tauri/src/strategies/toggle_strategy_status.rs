use crate::AppState;

#[tauri::command]
pub async fn toggle_strategy_status(
    state: tauri::State<'_, AppState>,
    strategy_id: u16,
) -> Result<(), String> {
    let db = &state.db;
    // First get the current status
    let current_status =
        sqlx::query_scalar::<_, String>("SELECT status FROM strategies WHERE id = $1")
            .bind(strategy_id)
            .fetch_one(db)
            .await
            .map_err(|e| e.to_string())?;

    // Determine new status based on current status
    let new_status = match current_status.as_str() {
        "Executing" => "Disabled",
        "Disabled" => "Executing",
        "Executed" => return Err("Cannot update status of executed strategies".to_string()),
        _ => return Err("Invalid strategy status".to_string()),
    };

    // Update the status
    sqlx::query("UPDATE strategies SET status = $1 WHERE id = $2")
        .bind(new_status)
        .bind(strategy_id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
