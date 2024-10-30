mod accounts;
mod rpcs;
mod types;
mod utils;

pub use accounts::*;
pub use rpcs::*;
use tauri::Manager;
pub use types::*;
pub use utils::*;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn capitalize(s: &str) -> String {
    s.to_uppercase()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            capitalize,
            import_account,
            get_accounts,
            delete_account,
            toggle_account_status,
            update_account,
            get_account_balance,
            get_rpcs_endpoint,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let db = setup_db(&app).await;
    let rpc_client = setup_rpc_client();
    app.manage(AppState { db, rpc_client });
    app.run(|_, _| {});
}
