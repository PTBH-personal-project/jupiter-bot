use crate::AppState;

#[tauri::command]
pub async fn delete_account(
    state: tauri::State<'_, AppState>,
    account_id: u16,
) -> Result<(), String> {
    let db = &state.db;
    sqlx::query("DELETE FROM accounts WHERE id = $1")
        .bind(account_id)
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
