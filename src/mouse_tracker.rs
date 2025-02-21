use device_query::{DeviceQuery, DeviceState, MouseState};

/// Tracks the current mouse position.
pub fn track_mouse() -> (i32, i32) {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();
    (mouse.coords.0, mouse.coords.1)
}