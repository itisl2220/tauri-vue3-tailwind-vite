#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod error;
pub mod cookie;
pub mod utils;
pub mod audio;
use std::sync::Arc;

use tauri::Manager;
use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::utils::SledDb;


// #[tokio::main]
fn main() {
    // 日志初始化
    // 设置日志格式和时间
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .with_timer(local_time)
        .init();
    tracing::info!("程序启动");
    let ctx = tauri::generate_context!();
    // 请求初始化
    let reqwest_client = Arc::new(reqwest::Client::builder().cookie_store(true).build().unwrap());

    //KV数据库初始化
    let db: SledDb = SledDb::new();

    tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(
            tauri::generate_handler![])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let main_window = app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            app.manage(reqwest_client);
            app.manage(db);
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
