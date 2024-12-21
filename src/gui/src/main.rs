#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use image::ImageReader;
use std::{io::Cursor, sync::Arc};

const PROJECT_NAME: &str = "PhotosNorm";

fn load_icon() -> Option<Arc<egui::IconData>> {
    let image_reader = ImageReader::new(Cursor::new(include_bytes!("../assets/icon.png")))
        .with_guessed_format()
        .ok()?;
    match image_reader.decode() {
        Ok(image) => Some(Arc::new(egui::IconData {
            rgba: image.to_rgba8().to_vec(),
            width: image.width(),
            height: image.height(),
        })),
        Err(e) => {
            println!("Failed to load icon: {}", e);
            None
        }
    }
}

struct WelcomePannel {}

impl eframe::App for WelcomePannel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Welcome to {} !", PROJECT_NAME));
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            icon: load_icon(),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        PROJECT_NAME,
        native_options,
        Box::new(|_cc| Ok(Box::new(WelcomePannel {}))),
    )
}
