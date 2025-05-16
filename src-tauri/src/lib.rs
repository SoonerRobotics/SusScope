use std::sync::Mutex;

use native_dialog::{DialogBuilder};
use tauri::{Builder, Manager, State};

struct AppState {
    log_file: String
}

#[tauri::command]
fn select_log_file(state: State<'_, Mutex<AppState>>) -> String {
    // Open a file dialog to select the log file.
    let path = DialogBuilder::file()
        .set_location("~/Desktop")
        // .add_filter("SusScope Log File", &["suslog"])
        .open_single_file()
        .show()
        .unwrap();

    // Check if the user selected a file.
    if path.is_none() {
        return "".to_string();
    }

    // Update the state
    let path = path.unwrap();
    let path = path.to_str().unwrap();

    let mut state = state.lock().unwrap();
    state.log_file = path.to_string();

    return path.to_string();
}

#[tauri::command]
fn get_log_file(state: State<'_, Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    return state.log_file.clone();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(AppState {
            log_file: String::new()
        }))
        .invoke_handler(tauri::generate_handler![
            select_log_file,
            get_log_file
        ])
        .run(tauri::generate_context!())
        .expect("An error occured while running SusScope.");
}
