use eframe::egui;

const PROJECT_NAME: &str = "PhotosNorm";

struct WelcomePannel {}

impl eframe::App for WelcomePannel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Welcome to {} !", PROJECT_NAME));
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        PROJECT_NAME,
        native_options,
        Box::new(|_cc| Ok(Box::new(WelcomePannel {}))),
    )
}
