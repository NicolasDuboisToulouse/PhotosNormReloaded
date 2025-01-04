use eframe::egui;
use image::ImageReader;
use std::{io::Cursor, sync::Arc};

const APP_ICON: &[u8] = include_bytes!("../assets/icon.png");

pub const SPLASH_ON_LIGHT: egui::ImageSource = egui::include_image!("../assets/splash.png");
pub const SPLASH_ON_DARK: egui::ImageSource = egui::include_image!("../assets/splash_on_dark.png");
pub const ERROR_IMG: egui::ImageSource = egui::include_image!("../assets/error.png");

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

///
/// Configure egui context
///
pub fn init_egui_ctx(ctx: &egui::Context) {
    ctx.style_mut(|style| {
        style.wrap_mode = Some(egui::TextWrapMode::Extend);
        style.spacing.button_padding = egui::vec2(15.0, 2.0);
        style.interaction = eframe::egui::style::Interaction {
            tooltip_delay: 0.2,
            show_tooltips_only_when_still: false,
            ..Default::default()
        }
    });
    ctx.options_mut(|options| {
        options.max_passes = std::num::NonZeroUsize::new(15).unwrap();
    });
}
