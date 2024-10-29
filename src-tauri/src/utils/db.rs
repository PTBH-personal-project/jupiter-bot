use crate::types::*;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs::{self, OpenOptions};
use std::io;
use tauri::{App, Manager};

pub async fn setup_db(app: &App) -> Db {
    let mut path = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    match fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(e) => {
            panic!("Failed to create app data dir: {}", e);
        }
    }
    path.push("db.sqlite");
    let result = OpenOptions::new().create_new(true).write(true).open(&path);
    match result {
        Ok(_) => println!("database file created"),
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => println!("database file already existed"),
            _ => {
                panic!("Failed to create database file: {}", e);
            }
        },
    }

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&db).await.unwrap();
    db
}
