use crate::types::*;

#[tauri::command]
pub async fn delete_token(
    state: tauri::State<'_, AppState>,
    address: String,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query(
        "DELETE FROM tokens WHERE address = ?"
    )
    .bind(address)
    .execute(db)
    .await
    .map_err(|e| format!("Failed to delete token: {}", e))?;

    Ok(())
}