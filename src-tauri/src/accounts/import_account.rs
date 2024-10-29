use crate::types::*;

#[tauri::command]
pub async fn import_account(
    state: tauri::State<'_, AppState>,
    account_name: String,
    private_key: String,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query(
        "INSERT INTO accounts (name, private_key, public_key, description, status) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(account_name)
    .bind(private_key)
    .bind("Public key")
    .bind("Description")
    .bind(AccountStatus::Enabled)
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}
