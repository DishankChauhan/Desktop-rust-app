// src/mouse_tracker.rs
use device_query::{DeviceQuery, DeviceState, MouseState};
use std::time::{SystemTime, UNIX_EPOCH};

/// Tracks the current mouse position.
pub fn track_mouse() -> (i32, i32) {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();
    (mouse.coords.0, mouse.coords.1)
}

/// Returns the current timestamp in seconds since the Unix epoch.
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}