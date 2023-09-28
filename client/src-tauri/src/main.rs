// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate brygge;

#[tokio::main]
async fn main() {
    println!("Starting Dreka client");

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                // Start server in separate process
                let response = brygge::start().await;
                if let Err(err) = response {
                    println!("Server: error: {}", err)
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
