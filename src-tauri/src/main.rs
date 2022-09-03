#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use audio::monitor;
use log::warn;
use tauri::{Manager, State};

mod audio;

const MIC_THRESHOLD: f32 = 0.5f32;

struct MicThreshold(Arc<Mutex<f32>>);

fn main() {
    let threshold = Arc::new(Mutex::new(MIC_THRESHOLD));

    tauri::Builder::default()
        .manage(MicThreshold(threshold.clone()))
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                monitor(window, threshold).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_mic_threshold,
            get_mic_threshold
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn set_mic_threshold(threshold: f32, current: State<'_, MicThreshold>) {
    *current.0.lock().unwrap() = threshold;
}

#[tauri::command]
fn get_mic_threshold(current: State<'_, MicThreshold>) -> f32 {
    *current.0.lock().unwrap()
}
