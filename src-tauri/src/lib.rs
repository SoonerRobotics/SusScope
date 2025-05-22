use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{
    fs::File,
    io::Read,
    process::{Command, Stdio},
};

use log::{error, info};
use native_dialog::DialogBuilder;
use tauri::ipc::Response;
use tauri::{Builder, State};
use tempfile::tempdir;
use zip::ZipArchive;

use http::{header::*, response::Builder as ResponseBuilder, status::StatusCode};
use tauri_plugin_log::{Target, TargetKind};

struct AppState {
    log_file: String,
}

#[tauri::command]
fn select_log_file(state: State<'_, Mutex<AppState>>) -> String {
    // Open a file dialog to select the log file.
    let path = DialogBuilder::file()
        .set_location("~/Desktop")
        .add_filter("SusScope Zip File", &["zip"])
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

#[tauri::command]
fn read_log_file(state: State<'_, Mutex<AppState>>) -> Response {
    let state = state.lock().unwrap();
    let log_file = &state.log_file;

    // Check if the log file is empty
    if log_file.is_empty() {
        return tauri::ipc::Response::new("".to_string());
    }

    // Read the log file
    let file = fs::File::open(log_file);
    if file.is_err() {
        return tauri::ipc::Response::new("".to_string());
    }

    // Create a new ZipArchive from the file
    let archive = zip::ZipArchive::new(file.unwrap());
    if archive.is_err() {
        return tauri::ipc::Response::new("".to_string());
    }

    // Find the file named "output.suslog" in the archive
    let mut binding = archive.unwrap();
    let mut file = match binding.by_name("output.suslog") {
        Ok(file) => file,
        Err(_) => return tauri::ipc::Response::new("".to_string()),
    };

    // Read the contents of the file into a string
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return tauri::ipc::Response::new("".to_string());
    }

    return tauri::ipc::Response::new(contents);
}

#[tauri::command]
fn read_video(video: String, state: State<'_, Mutex<AppState>>) -> String {
    use dirs::cache_dir;
    use std::path::{Path, PathBuf};

    let state = state.lock().unwrap();
    let log_file = &state.log_file;

    info!("Attempting to read video: {}", video);

    if log_file.is_empty() {
        error!("Log file path is empty");
        return String::new();
    }

    // Open ZIP archive
    let file = match fs::File::open(log_file) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open log file: {}", e);
            return String::new();
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => {
            error!("Failed to open ZIP archive: {}", e);
            return String::new();
        }
    };

    // Get video from ZIP
    let mut file = match archive.by_name(&video) {
        Ok(f) => f,
        Err(e) => {
            error!("Video '{}' not found in archive: {}", video, e);
            return String::new();
        }
    };

    // Get persistent cache dir
    let cache_path = match cache_dir() {
        Some(path) => path.join("susscope_videos"),
        None => {
            error!("Failed to get cache directory");
            return String::new();
        }
    };

    if let Err(e) = fs::create_dir_all(&cache_path) {
        error!("Failed to create cache directory: {}", e);
        return String::new();
    }

    let base_name = Path::new(&video)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let avi_path = cache_path.join(format!("{base_name}.avi"));
    let mp4_path = cache_path.join(format!("{base_name}.mp4"));

    // If MP4 already exists, return it
    if mp4_path.exists() {
        info!("MP4 already exists at: {:?}", mp4_path);
        return mp4_path.to_string_lossy().to_string();
    }

    // Write AVI to cache
    match File::create(&avi_path).and_then(|mut out| std::io::copy(&mut file, &mut out)) {
        Ok(_) => info!("AVI file written to: {:?}", avi_path),
        Err(e) => {
            error!("Failed to write AVI file: {}", e);
            return String::new();
        }
    }

    // Convert to MP4
    info!("Starting ffmpeg conversion...");
    let status = Command::new("ffmpeg")
        .args(["-i", avi_path.to_str().unwrap(), mp4_path.to_str().unwrap()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(s) if s.success() => {
            info!("Conversion successful: {:?}", mp4_path);
            mp4_path.to_string_lossy().to_string()
        }
        Ok(s) => {
            error!("ffmpeg failed with code: {}", s);
            String::new()
        }
        Err(e) => {
            error!("Failed to execute ffmpeg: {}", e);
            String::new()
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .register_uri_scheme_protocol("mp4", move |_ctx, request| {
            let path_encoded = request.uri().path()[1..].to_string();
            info!("Request for video: {}", path_encoded);

            // Convert the path from url encoded
            let path = match percent_encoding::percent_decode_str(&path_encoded).decode_utf8() {
                Ok(p) => p.to_string(),
                Err(e) => {
                    error!("Failed to decode URL: {}", e);
                    return ResponseBuilder::new()
                        .status(StatusCode::BAD_REQUEST)
                        .header(CONTENT_TYPE, "text/plain")
                        .body("Invalid URL".as_bytes().to_vec())
                        .unwrap();
                }
            };

            // Read the video file
            let mut file = match File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    error!("Failed to open video file: {}", e);
                    return ResponseBuilder::new()
                        .status(StatusCode::NOT_FOUND)
                        .header(CONTENT_TYPE, "text/plain")
                        .body("Video not found".as_bytes().to_vec())
                        .unwrap();
                }
            };

            // Read the file into a buffer
            let mut buffer = Vec::new();
            if file.read_to_end(&mut buffer).is_err() {
                error!("Failed to read video file");
                return ResponseBuilder::new()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(CONTENT_TYPE, "text/plain")
                    .body("Failed to read video file".as_bytes().to_vec())
                    .unwrap();
            }

            // Create a response
            let response = ResponseBuilder::new()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "video/mp4")
                .header("Accept-Ranges", "bytes")
                .header("Content-Length", buffer.len().to_string())
                .header("Content-Range", format!("bytes 0-{}/{}", buffer.len(), buffer.len() + 1))
                .body(buffer)
                .unwrap();

            // Send the response
            response
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .manage(Mutex::new(AppState {
            log_file: String::new(),
        }))
        .invoke_handler(tauri::generate_handler![
            select_log_file,
            get_log_file,
            read_log_file,
            read_video,
        ])
        .run(tauri::generate_context!())
        .expect("An error occured while running SusScope.");
}
