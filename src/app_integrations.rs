// src/app_integrations.rs
use std::process::Command;

pub fn open_figma() {
    Command::new("open")
        .arg("-a")
        .arg("Figma")
        .spawn()
        .expect("Failed to open Figma");
}

pub fn open_zoom() {
    Command::new("open")
        .arg("-a")
        .arg("Zoom")
        .spawn()
        .expect("Failed to open Zoom");
}