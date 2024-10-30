use crate::types::*;

#[tauri::command]
pub async fn import_rpc(
    state: tauri::State<'_, AppState>,
    name: String,
    url: String,
    description: String,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query("INSERT INTO rpcs (name, url, description) VALUES ($1, $2, $3)")
        .bind(&name)
        .bind(&url)
        .bind(&description)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to import RPC: {}", e))?;

    Ok(())
}
