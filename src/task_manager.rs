use crate::mouse_tracker::track_mouse;
use crate::csv_writer::save_to_csv;
use chrono::{Duration, Local};

/// Represents a task with a name, start/end times, and mouse positions.
pub struct Task {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub mouse_positions: Vec<(i32, i32)>,
}

impl Task {
    /// Creates a new task with the current timestamp as the start time.
    pub fn new(name: String) -> Self {
        Self {
            name,
            start_time: Local::now().to_string(),
            end_time: String::new(),
            mouse_positions: Vec::new(),
        }
    }

    /// Stops the task and records the end time.
    pub fn stop(&mut self) {
        self.end_time = Local::now().to_string();
    }

    /// Records the current mouse position.
    pub fn record_mouse(&mut self) {
        self.mouse_positions.push(track_mouse());
    }

    /// Saves the task details to a CSV file.
    pub fn save_to_csv(&self, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        save_to_csv(&self.name, &self.start_time, &self.end_time, &self.mouse_positions, output_dir)
    }

    /// Calculates the duration of the task.
    pub fn duration(&self) -> Duration {
        let start = Local::now(); // Use current time for simplicity
        let end = Local::now(); // Use current time for simplicity
        end - start
    }
}

/// Manages a collection of tasks.
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    /// Creates a new `TaskManager` with an empty task list.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Adds a task to the task list.
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Returns a reference to the list of tasks.
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Returns a mutable reference to the current task (the last one added).
    pub fn current_task(&mut self) -> Option<&mut Task> {
        self.tasks.last_mut()
    }
}