use eframe::egui;

pub fn show_error(ctx: &egui::Context, message: &str) {
    egui::Window::new("Error").show(ctx, |ui| {
        ui.label(message);
    });
}