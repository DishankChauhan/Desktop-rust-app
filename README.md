# Rust Desktop App

A desktop application built in Rust that tracks mouse movements, manages tasks, integrates with external apps (e.g., browser, Figma, Zoom), and records the screen. The app saves mouse positions and task details to a CSV file.

---

## Features

- **Task Management**: Create and stop tasks, track task duration, and save task details to a CSV file.
- **Mouse Tracking**: Track mouse movements in real-time and visualize them with a trail.
- **Screen Recording**: Start and stop screen recording using `screencapture`.
- **App Integrations**: Open browser, Figma, and Zoom with error handling.
- **Customizable Settings**: Choose between light/dark mode, customize mouse trail color and size, and set the default recording directory.
- **Notifications**: Desktop notifications for task/recording events and errors.

---

## Screenshot

![Screenshot](screenshot.png) <!-- Add a screenshot here -->

---

## Prerequisites

Before running the app, ensure you have the following installed:

1. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/).
2. **FFmpeg** (optional): Required for screen recording using `ffmpeg`. Install it using Homebrew:
   ```bash
   brew install ffmpeg