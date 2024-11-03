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

#[tauri::command]
pub async fn get_account_by_public_key(
    state: tauri::State<'_, AppState>,
    public_key: String,
) -> Result<Account, String> {
    let db = &state.db;

    let account = sqlx::query_as::<_, Account>(
        "SELECT * FROM accounts WHERE public_key = ?"
    )
    .bind(public_key)
    .fetch_optional(db)
    .await
    .map_err(|e| format!("Failed to fetch account: {}", e))?
    .ok_or_else(|| "Account not found".to_string())?;

    Ok(account)
}