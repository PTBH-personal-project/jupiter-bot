use crate::types::*;

#[tauri::command]
pub async fn import_token(
    state: tauri::State<'_, AppState>,
    address: String,
    symbol: Option<String>,
    decimals: i64,
    name: Option<String>,
    logo_uri: Option<String>,
    uri: String,
) -> Result<(), String> {
    let db = &state.db;

    sqlx::query(
        "INSERT INTO tokens (address, symbol, decimals, name, logo_uri, uri) 
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(address)
    .bind(symbol)
    .bind(decimals)
    .bind(name)
    .bind(logo_uri)
    .bind(uri)
    .execute(db)
    .await
    .map_err(|e| format!("Failed to import token: {}", e))?;

    Ok(())
}
