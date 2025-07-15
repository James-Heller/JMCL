#[tauri::command]
pub fn has_minecraft_folder() -> bool {
    let current_dir = std::env::current_dir().unwrap();
    let minecraft_path = current_dir.join(".minecraft");
    return minecraft_path.exists() && minecraft_path.is_dir();
}

#[tauri::command]
pub fn current() -> String {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.to_str().unwrap().to_string()
}
