// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod dir_scanner;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![dir_scanner::has_minecraft_folder, dir_scanner::current])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

