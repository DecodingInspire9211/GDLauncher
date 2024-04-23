// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::Menu;

fn main() {
    //let menu = Menu::new();

    tauri::Builder::default()
        //.menu(&menu)
        .setup(|app| {
            println!("{}", app.package_info().version.to_string());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Tauri application could not be built!");
}
