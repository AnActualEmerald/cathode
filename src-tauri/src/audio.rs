use std::sync::Arc;
use std::sync::Mutex;

use anyhow::anyhow;
use anyhow::Result;
use cpal::traits::StreamTrait;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Device;
use cpal::InputCallbackInfo;
// use cpal::OutputCallbackInfo;
use tauri::Window;

pub async fn monitor(window: Window, threshold: Arc<Mutex<f32>>) {
    let device = initialize().expect("Unable to init audio");
    println!("Using device {}", device.name().unwrap());
    let config = device.default_input_config().unwrap();
    let stream = device
        .build_input_stream(
            &config.config(),
            move |data: &[f32], _: &InputCallbackInfo| {
                if data.iter().any(|e| e.abs() >= *threshold.lock().unwrap()) {
                    window.emit("mouth-open", "").unwrap();
                } else {
                    window.emit("mouth-close", "").unwrap();
                }
            },
            move |err| {
                println!("Audio error: {:?}", err);
            },
        )
        .unwrap();

    println!("Start stream");
    stream.play().expect("Error creating output stream");
    loop {} // the stream will close if it goes out of scope, so just dwell here
}

fn initialize() -> Result<Device> {
    let host = cpal::default_host();
    host.default_input_device()
        .ok_or_else(|| anyhow!("No default output device"))
}
