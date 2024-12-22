use crate::assets;
use eframe::egui;

pub struct WelcomePannel {}

impl eframe::App for WelcomePannel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match ctx.theme() {
            egui::Theme::Light => {
                ui.image(assets::SPLASH_ON_LIGHT);
            }
            egui::Theme::Dark => {
                ui.image(assets::SPLASH_ON_DARK);
            }
        });
    }
}
