mod accounts;
mod types;
mod utils;

pub use accounts::*;
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
            get_accounts
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let db = setup_db(&app).await;
    app.manage(AppState { db });
    app.run(|_, _| {});
}
