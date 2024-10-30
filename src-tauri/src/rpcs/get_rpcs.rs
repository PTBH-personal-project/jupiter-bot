use crate::types::*;

#[tauri::command]
pub async fn get_rpcs_endpoint(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RpcEndpoint>, String> {
    let db = &state.db;
    let rpcs = sqlx::query_as::<_, RpcEndpoint>("SELECT * FROM rpcs")
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to get RPCs: {}", e))?;

    Ok(rpcs)
}
