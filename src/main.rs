mod mouse_tracker;
mod task_manager;
mod csv_writer;
mod browser;
mod app_integrations;
mod recording;
mod error_handler;

use eframe::egui;
use recording::stop_recording;
use crate::mouse_tracker::track_mouse;
use crate::task_manager::{Task, TaskManager};

use crate::browser::open_browser;
use crate::app_integrations::{open_figma, open_zoom};
use crate::recording::start_recording;
use crate::error_handler::show_error;

use std::fs::File;
use std::io::Write;
use notify_rust::Notification;

fn main() {
    let options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "Rust Desktop App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ) {
        eprintln!("Failed to run application: {}", e);
    }
}

struct MyApp {
    task_name: String,
    is_recording: bool,
    mouse_position: (i32, i32),
    task_manager: TaskManager,
    recording_output_dir: String,
    mouse_trail: Vec<(f32, f32)>,
    mouse_trail_color: egui::Color32,
    mouse_trail_size: f32,
    is_dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            task_name: String::new(),
            is_recording: false,
            mouse_position: (0, 0),
            task_manager: TaskManager::new(),
            recording_output_dir: String::from("."),
            mouse_trail: Vec::new(),
            mouse_trail_color: egui::Color32::RED,
            mouse_trail_size: 2.0,
            is_dark_mode: true,
        }
    }
}

impl MyApp {
    fn send_notification(&self, message: &str) {
        Notification::new()
            .summary("Rust Desktop App")
            .body(message)
            .show()
            .unwrap();
    }

    fn save_mouse_trail_as_image(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create("mouse_trail.txt")?;
        for (x, y) in &self.mouse_trail {
            writeln!(file, "{},{}", x, y)?;
        }
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update mouse position
        self.mouse_position = track_mouse();

        // Record mouse movements if a task is active
        if self.is_recording {
            if let Some(ref mut task) = self.task_manager.current_task() {
                task.record_mouse();
            }
        }

        // Add current mouse position to the trail
        self.mouse_trail.push((self.mouse_position.0 as f32, self.mouse_position.1 as f32));

        // Limit the trail length
        if self.mouse_trail.len() > 100 {
            self.mouse_trail.remove(0);
        }

        // Set GUI theme
        let mut style = (*ctx.style()).clone();
        if self.is_dark_mode {
            style.visuals = egui::Visuals::dark();
        } else {
            style.visuals = egui::Visuals::light();
        }
        ctx.set_style(style);

        // Top panel with buttons
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🎯 Rust Desktop App");
                ui.label("Task Name:");
                ui.text_edit_singleline(&mut self.task_name);
                if ui.button("🛠️ Create Task").clicked() {
                    println!("Creating task: {}", self.task_name);
                    self.is_recording = true;
                    let task = Task::new(self.task_name.clone());
                    self.task_manager.add_task(task);
                    println!("Starting recording in directory: {}", self.recording_output_dir);
                    if let Err(e) = start_recording(&self.recording_output_dir) {
                        println!("Failed to start recording: {}", e);
                        show_error(ctx, &format!("Failed to start recording: {}", e));
                        self.send_notification(&format!("Failed to start recording: {}", e));
                    } else {
                        println!("Recording started successfully.");
                        self.send_notification("Task started and recording began.");
                    }
                }
                if ui.button("⏹️ Stop Task").clicked() {
                    self.is_recording = false;
                    if let Some(task) = self.task_manager.current_task() {
                        task.stop();
                        if let Err(e) = task.save_to_csv(&self.recording_output_dir) {
                            show_error(ctx, &format!("Failed to save task: {}", e));
                            self.send_notification(&format!("Failed to save task: {}", e));
                        }
                        if let Err(e) = stop_recording() {
                            show_error(ctx, &format!("Failed to stop recording: {}", e));
                            self.send_notification(&format!("Failed to stop recording: {}", e));
                        } else {
                            self.send_notification("Task stopped and recording ended.");
                        }
                    }
                }
            });
        });

        // Side panel for task history and settings
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("📜 Task History");
            for task in self.task_manager.tasks() {
                ui.label(format!(
                    "{} - Duration: {}s",
                    task.name,
                    task.duration().num_seconds()
                ));
            }
        
            ui.separator();
        
            ui.heading("🚀 Quick Actions");
            if ui.button("🌐 Open Browser").clicked() {
                if let Err(e) = open_browser("https://www.google.com") {
                    show_error(ctx, &format!("Failed to open browser: {}", e));
                    self.send_notification(&format!("Failed to open browser: {}", e));
                }
            }
            if ui.button("🎨 Open Figma").clicked() {
                if let Err(e) = open_figma() {
                    show_error(ctx, &format!("Failed to open Figma: {}", e));
                    self.send_notification(&format!("Failed to open Figma: {}", e));
                }
            }
            if ui.button("📹 Open Zoom").clicked() {
                if let Err(e) = open_zoom() {
                    show_error(ctx, &format!("Failed to open Zoom: {}", e));
                    self.send_notification(&format!("Failed to open Zoom: {}", e));
                }
            }
        
            ui.separator();
        
            ui.heading("⚙️ Settings");
            ui.checkbox(&mut self.is_dark_mode, "Dark Mode");
            ui.color_edit_button_srgba(&mut self.mouse_trail_color);
            ui.add(egui::Slider::new(&mut self.mouse_trail_size, 1.0..=10.0).text("Trail Size"));
            ui.label("📂 Recording Output Directory:");
            ui.text_edit_singleline(&mut self.recording_output_dir);
        
            ui.separator();
        
            if ui.button("🧹 Clear Trail").clicked() {
                self.mouse_trail.clear();
            }
            if ui.button("💾 Save Trail as Image").clicked() {
                if let Err(e) = self.save_mouse_trail_as_image() {
                    show_error(ctx, &format!("Failed to save trail: {}", e));
                    self.send_notification(&format!("Failed to save trail: {}", e));
                } else {
                    self.send_notification("Mouse trail saved as image.");
                }
            }
        });

        // Central panel for mouse tracking and recording status
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🖱️ Mouse Tracker");
            ui.label(format!("📍 Mouse Position: ({}, {})", self.mouse_position.0, self.mouse_position.1));
        
            // Draw a small canvas to visualize mouse movements
            let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(200.0, 200.0), egui::Sense::hover());
            let painter = ui.painter();
        
            // Background
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(50, 50, 50));
        
            // Map screen coordinates to canvas coordinates
            let canvas_center = rect.center();
            let mouse_x = self.mouse_position.0 as f32 - canvas_center.x + 100.0; // Adjust for canvas size
            let mouse_y = self.mouse_position.1 as f32 - canvas_center.y + 100.0; // Adjust for canvas size
        
            // Draw the mouse trail
            for (i, (x, y)) in self.mouse_trail.iter().enumerate() {
                let alpha = (i as f32 / self.mouse_trail.len() as f32) * 255.0;
                painter.circle_filled(
                    canvas_center + egui::Vec2::new(*x - canvas_center.x + 100.0, *y - canvas_center.y + 100.0),
                    self.mouse_trail_size,
                    egui::Color32::from_rgba_premultiplied(
                        self.mouse_trail_color.r(),
                        self.mouse_trail_color.g(),
                        self.mouse_trail_color.b(),
                        alpha as u8,
                    ),
                );
            }
        
            // Draw the red dot
            painter.circle_filled(
                canvas_center + egui::Vec2::new(mouse_x, mouse_y),
                5.0,
                egui::Color32::RED,
            );
        
            if self.is_recording {
                ui.colored_label(egui::Color32::RED, "🔴 Recording");
            } else {
                ui.colored_label(egui::Color32::GREEN, "🟢 Not Recording");
            }
        });
    }
}