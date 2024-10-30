use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn delete_rpc(state: State<'_, AppState>, rpc_id: i32) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("DELETE FROM rpcs WHERE id = $1")
        .bind(&rpc_id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
