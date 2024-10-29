use crate::types::*;

#[tauri::command]
pub async fn get_accounts(state: tauri::State<'_, AppState>) -> Result<Vec<Account>, String> {
    let db = &state.db;
    let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get accounts: {}", e))?;

    Ok(accounts)
}
