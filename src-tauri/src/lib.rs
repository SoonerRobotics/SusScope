use native_dialog::{DialogBuilder};

#[tauri::command]
fn select_log_file() -> String {
    // Open a file dialog to select the log file.
    let path = DialogBuilder::file()
        .set_location("~/Desktop")
        .add_filter("SusScope Log File", &["suslog"])
        .open_single_file()
        .show()
        .unwrap();

    format!("{:?}", path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![select_log_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
