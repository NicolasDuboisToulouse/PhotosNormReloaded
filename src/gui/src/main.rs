#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod assets;
mod taffy_tools;
mod welcome_pannel;
use eframe::egui;
use welcome_pannel::WelcomePannel;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            icon: assets::load_app_icon(),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        assets::PROJECT_NAME,
        native_options,
        Box::new(|cc| {
            //            cc.egui_ctx.set_theme(egui::ThemePreference::Light);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<WelcomePannel>::default())
        }),
    )
}
