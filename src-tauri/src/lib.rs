use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;

#[tauri::command]
fn greet(name: &str) {
    let new_user = user {
        name: "test",
        distance: 32.,
        points: 300
    };
    write_data(&new_user);
}

#[derive(Serialize, Deserialize, Debug)]
struct user {
    name: &'static str,
    distance: f32,
    points: i32
}

fn write_data(user_data: &user) {
    let serialized = serde_json::to_string(&user_data).unwrap();
    fs::create_dir_all("../config");
    let mut f = File::create("../config/config.json").expect("unable to create file");
    f.write_all(serialized.as_bytes()).expect("Unable to write to file.");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
