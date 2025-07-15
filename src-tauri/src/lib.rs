// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod database;
mod dir_scanner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    database::check_db_exists();
    database::run_db_setup();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            dir_scanner::has_minecraft_folder,
            dir_scanner::current,
            database::get_installed_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
