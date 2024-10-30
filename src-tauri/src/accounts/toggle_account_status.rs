use crate::AppState;

#[tauri::command]
pub async fn toggle_account_status(
    state: tauri::State<'_, AppState>,
    account_id: u16,
    new_status: String,
) -> Result<(), String> {
    let db = &state.db;
    sqlx::query("UPDATE accounts SET status = $1 WHERE id = $2")
        .bind(new_status)
        .bind(account_id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
