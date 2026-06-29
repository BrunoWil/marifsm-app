// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Emitter};

/// Comando para abrir uma nova janela de login da Twitch.
#[tauri::command]
async fn open_login_window(app: tauri::AppHandle) -> Result<(), tauri::Error> {
    let _login_window = tauri::WebviewWindowBuilder::new(
        &app.clone(),
        "login",
        tauri::WebviewUrl::External("https://www.twitch.tv/login".parse().unwrap()),
    )
    .on_navigation(move |url| {
        if url.host_str() == Some("www.twitch.tv") && !url.path().starts_with("/login") {
            if let Some(window) = app.get_webview_window("login") {
                let _ = window.close();
            }
            let _ = app.emit("login-successful", ());
        }
        true
    })
    .title("Login - Twitch")
    .inner_size(400.0, 600.0)
    .resizable(false)
    .always_on_top(true)
    .build()?;

    Ok(())
}

fn main() {
  // A inicialização padrão do Tauri Builder lê o `tauri.conf.json`
  // e cria as janelas definidas lá, sem a necessidade de código manual.
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_login_window])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}