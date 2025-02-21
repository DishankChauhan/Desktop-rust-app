// src/task_manager.rs
use crate::mouse_tracker::{track_mouse, get_timestamp};
use crate::csv_writer::save_to_csv;

pub struct Task {
    pub name: String,
    pub end_time: u64,
    pub mouse_positions: Vec<(i32, i32)>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            end_time: 0,
            mouse_positions: Vec::new(),
        }
    }

    pub fn stop(&mut self) {
        self.end_time = get_timestamp();
    }

    pub fn record_mouse(&mut self) {
        self.mouse_positions.push(track_mouse());
    }

    pub fn save_to_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
        save_to_csv(&self.name, &self.mouse_positions)
    }
}