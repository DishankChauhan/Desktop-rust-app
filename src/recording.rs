// src/recording.rs
use std::process::Command;

pub fn start_recording(output_file: &str) {
    Command::new("ffmpeg")
        .arg("-f")
        .arg("avfoundation")
        .arg("-i")
        .arg("1:0") // 1:0 is the default screen capture device on macOS
        .arg(output_file)
        .spawn()
        .expect("Failed to start recording");
}

pub fn stop_recording() {
    Command::new("pkill")
        .arg("ffmpeg")
        .spawn()
        .expect("Failed to stop recording");
}