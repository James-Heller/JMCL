use serde::Serialize;
use sqlite::State;
#[derive(Serialize)]
struct GameVersion {
    id: i64,
    name: String,
    version: String,
    mod_loader: String,
    location: String,
}
pub fn check_db_exists() -> bool {
    sqlite::open(".launcher.db").is_ok()
}

pub fn run_db_setup() -> bool {
    let installed_game_version_table = "CREATE TABLE IF NOT EXISTS game_version \
    ( id INTEGER PRIMARY KEY, name VARCHAR(32), version VARCHAR(64), mod_loader VARCHAR(255), location TEXT);";

    let connection = sqlite::open(".launcher.db").unwrap();

    connection.execute(installed_game_version_table).is_ok()
}

#[tauri::command]
pub fn get_installed_game() -> String {
    let connection = sqlite::open(".launcher.db").unwrap();
    let mut statement = connection.prepare("SELECT * FROM game_version").unwrap();

    let mut list: Vec<GameVersion> = Vec::new();
    while let State::Row = statement.next().unwrap() {
        let id: i64 = statement.read(0).unwrap();
        let name: String = statement.read(1).unwrap();
        let version: String = statement.read(2).unwrap();
        let mod_loader: String = statement.read(3).unwrap();
        let location: String = statement.read(4).unwrap();
        let game_version: GameVersion = GameVersion {
            id,
            name,
            version,
            mod_loader,
            location,
        };
        list.push(game_version);
    }

    if list.len() == 0 {
        list.push(GameVersion {
            id: 0,
            name: "测试数据".to_string(),
            version: "1.12.2".to_string(),
            mod_loader: "Forge".to_string(),
            location: ".minecraft".to_string(),
        })
    }

    return serde_json::to_string(&list).unwrap();
}
