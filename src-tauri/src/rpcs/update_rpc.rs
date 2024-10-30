use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn update_rpc(
    state: State<'_, AppState>,
    rpc_id: i32,
    name: String,
    url: String,
    description: String,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("UPDATE rpcs SET name = $1, url = $2, description = $3 WHERE id = $4")
        .bind(&name)
        .bind(&url)
        .bind(&description)
        .bind(&rpc_id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
