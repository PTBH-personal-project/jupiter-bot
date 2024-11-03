use crate::types::*;

#[tauri::command]
pub async fn get_all_tokens(state: tauri::State<'_, AppState>) -> Result<Vec<TokenInfo>, String> {
    let db = &state.db;

    let tokens = sqlx::query_as::<_, TokenInfo>(
        r#"
        SELECT *
        FROM tokens
        "#
    )
    .fetch_all(db)
    .await
    .map_err(|e| format!("Failed to fetch tokens: {}", e))?;

    Ok(tokens)
}
