use eframe::egui;
use image::ImageReader;
use std::{io::Cursor, sync::Arc};

///
/// Define the main application name (can we store that elsewhere ?)
///
pub const PROJECT_NAME: &str = "PhotosNorm";

const APP_ICON: &[u8] = include_bytes!("../assets/icon.png");

pub const SPLASH_ON_LIGHT: egui::ImageSource = egui::include_image!("../assets/splash.png");
pub const SPLASH_ON_DARK: egui::ImageSource = egui::include_image!("../assets/splash_on_dark.png");

///
/// Load main application icon
///
pub fn load_app_icon() -> Option<Arc<egui::IconData>> {
    let image_reader = ImageReader::new(Cursor::new(APP_ICON))
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
