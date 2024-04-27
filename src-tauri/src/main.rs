// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{webview::PageLoadEvent, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::process::TerminatedPayload;

// #[tauri::command]
// fn open_file_dialog<R: Runtime>(
//     app: tauri::AppHandle<R>,
//     window: tauri::Window<R>,
// ) -> Result<(), String> {
//     let file_path = app
//         .dialog()
//         .file()
//         .add_filter("Godot Project", &["project.godot"])
//         .add_filter("ZIP", &[".zip"])
//         .blocking_pick_file();
//     Ok(())
// }

#[tauri::command]
fn get_os_info() -> String {
    format!(
        "{} on {}",
        tauri_plugin_os::hostname().to_string(),
        tauri_plugin_os::platform().to_string()
    )
}

#[derive(Clone, serde::Serialize)]
struct AppMetadata {
    ver: String,
    auth: String,
}

fn main() {
    //let menu = Menu::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        //.menu(&menu)
        .setup(|app| {
            let version = format!("{}", app.package_info().version.to_string());
            let author = format!("{}", app.package_info().authors.to_string());

            app.emit(
                "app-info",
                AppMetadata {
                    ver: version.into(),
                    auth: author.into(),
                },
            )
            .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_os_info])
        .run(tauri::generate_context!())
        .expect("Tauri application could not be built!");
}
