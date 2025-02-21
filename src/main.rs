// src/main.rs
mod mouse_tracker;
mod task_manager;
mod csv_writer;
mod browser;
mod app_integrations;
mod recording;

use eframe::egui;
use crate::task_manager::Task;

use crate::browser::open_browser;
use crate::app_integrations::{open_figma, open_zoom};
use crate::recording::{start_recording, stop_recording};

fn main() {
    let options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "Rust Desktop App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ) {
        eprintln!("Application error: {}", e);
    }
}

struct MyApp {
    task_name: String,
    is_recording: bool,
    current_task: Option<Task>, // Track the current task
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            task_name: String::new(),
            is_recording: false,
            current_task: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Rust Desktop App");

                // Create Task Button
                if ui.button("Create Task").clicked() {
                    self.is_recording = true;
                    self.current_task = Some(Task::new(self.task_name.clone()));
                    start_recording("output.mp4"); // Start screen recording
                    println!("Task started: {}", self.task_name);
                }

                // Stop Task Button
                if ui.button("Stop Task").clicked() {
                    self.is_recording = false;
                    if let Some(mut task) = self.current_task.take() {
                        task.stop();
                        task.save_to_csv().expect("Failed to save task to CSV");
                        stop_recording(); // Stop screen recording
                        println!("Task stopped: {}", self.task_name);
                    }
                }

                // Open Browser Button
                if ui.button("Open Browser").clicked() {
                    open_browser("https://www.google.com").expect("Failed to open browser");
                }

                // Open Figma Button
                if ui.button("Open Figma").clicked() {
                    open_figma();
                }

                // Open Zoom Button
                if ui.button("Open Zoom").clicked() {
                    open_zoom();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Task Name:");
            ui.text_edit_singleline(&mut self.task_name);

            // Record mouse movements if a task is active
            if self.is_recording {
                if let Some(ref mut task) = self.current_task {
                    task.record_mouse();
                }
            }
        });
    }
}