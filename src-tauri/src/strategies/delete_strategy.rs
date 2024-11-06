use crate::types::*;

#[tauri::command]
pub async fn delete_strategy(
    state: tauri::State<'_, AppState>,
    strategy_id: i32,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("DELETE FROM strategies WHERE id = $1")
        .bind(strategy_id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete strategy: {}", e))?;

    Ok(())
}
