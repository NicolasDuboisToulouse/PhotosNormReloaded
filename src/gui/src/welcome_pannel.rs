use crate::assets;
use eframe::egui;

pub struct WelcomePannel {}

impl eframe::App for WelcomePannel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Welcome to {} !", assets::PROJECT_NAME));
        });
    }
}
