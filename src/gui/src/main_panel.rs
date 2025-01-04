use crate::{assets, taffy_tools};
use core::assets as core_assets;
use eframe::egui;
use egui_taffy::{self, taffy, TuiBuilderLogic};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

struct ImageData<'a> {
    #[allow(dead_code)] // May be used later
    path: &'a PathBuf,
    uri: Result<String, std::io::Error>,
    name: String,
}

pub struct MainPanel<'a> {
    /// list of selected paths (result)
    images: Vec<ImageData<'a>>,
    initialized: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
struct State {
    images_size_vec: egui::Vec2,
}

impl Default for State {
    fn default() -> Self {
        State {
            images_size_vec: egui::Vec2::ZERO,
        }
    }
}

impl State {
    pub fn load(ctx: &egui::Context, id: egui::Id) -> Option<Self> {
        ctx.data_mut(|d| d.get_persisted(id))
    }

    pub fn store(self, ctx: &egui::Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_persisted(id, self));
    }
}

impl MainPanel<'_> {
    pub fn show(image_paths: &[PathBuf]) -> eframe::Result {
        // Compute ImageData for each image
        let images = image_paths
            .iter()
            .map(|path| {
                let uri = match std::fs::canonicalize(path) {
                    Ok(path) => match path.to_str() {
                        Some(path_str) => {
                            let mut uri = String::from("file://");
                            uri.push_str(path_str);
                            Ok(uri)
                        }
                        None => Err(std::io::Error::other("Unsupported non-UT8 paths.")),
                    },
                    Err(e) => Err(e),
                };

                let name = path
                    .file_name()
                    .unwrap_or(std::ffi::OsStr::new("Unexpected invalid file name"))
                    .to_string_lossy()
                    .to_string();

                ImageData { path, uri, name }
            })
            .collect::<Vec<_>>();

        let panel = MainPanel {
            images,
            initialized: false,
        };

        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                icon: assets::load_app_icon(),
                ..Default::default()
            },
            ..Default::default()
        };

        eframe::run_native(
            core_assets::PROJECT_NAME,
            native_options,
            Box::new(|cc| {
                //            cc.egui_ctx.set_theme(egui::ThemePreference::Light);
                egui_extras::install_image_loaders(&cc.egui_ctx);
                assets::init_egui_ctx(&cc.egui_ctx);

                Ok(Box::new(panel))
            }),
        )
    }
}

impl eframe::App for MainPanel<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut state = State::load(ctx, ui.id()).unwrap_or_else(|| {
                let images_size = ui.available_height() / 4.0;
                State {
                    images_size_vec: egui::Vec2::new(images_size, images_size),
                }
            });
            let images_size = state.images_size_vec.x;

            //
            // Header
            //
            egui_taffy::tui(ui, ui.id().with("main"))
                .reserve_available_space()
                .style(taffy::Style {
                    flex_direction: taffy::FlexDirection::Row,
                    padding: taffy::Rect {
                        left: taffy::LengthPercentage::Percent(0.02),
                        right: taffy::LengthPercentage::Percent(0.02),
                        top: taffy::LengthPercentage::Length(20.),
                        bottom: taffy::LengthPercentage::Length(10.),
                    },
                    gap: taffy::prelude::length(10.),
                    align_items: Some(taffy::AlignItems::Center),
                    size: taffy_tools::h_percent(1.0),
                    ..Default::default()
                })
                .show(|tui| {
                    tui.ui(|ui| {
                        if ui
                            .button("Save and fix all...")
                            .on_hover_ui(|ui| {
                                ui.label(
                                    "Save modified metadata and apply fixes (orientation, file name...)",
                                );
                            })
                            .clicked()
                        {
                            println!("TODO: implement Save and fix all");
                        }
                    });
                    tui.ui(|ui| {
                        if ui
                            .button("Save all...")
                            .on_hover_ui(|ui| {
                                ui.label("Only save modified metadata");
                            })
                            .clicked()
                        {
                            println!("TODO: implement Save");
                        }
                    });
                    tui.style(taffy::Style {
                        flex_grow: 1.,
                        ..Default::default()
                    })
                    .add_empty();
                    tui.label("Image size:");
                    tui.ui(|ui| {
                        ui.add(egui::Slider::from_get_set(100.0..=500.0, |value| match value {
                            None => images_size.into(),
                            Some(images_size) => {
                                state.images_size_vec =
                                    egui::Vec2::new(images_size as f32, images_size as f32);
                                images_size
                            }
                        }));
                    });
                });

            ui.separator();

            //
            // Image list
            //
            let scroll_area = if self.initialized {
                egui::ScrollArea::vertical()
            } else {
                // Clear scroll_offset persistance
                egui::ScrollArea::vertical().scroll_offset(egui::Vec2::ZERO)
            };
            scroll_area.show(ui, |ui| {
                egui_taffy::tui(ui, ui.id().with("main"))
                    .reserve_available_space()
                    .style(taffy::Style {
                        display: taffy::Display::Block,
                        padding: taffy_tools::h_percent(0.02),
                        size: taffy::prelude::percent(1.),
                        ..Default::default()
                    })
                    .show(|tui| {
                        for image in &self.images {
                            tui.style(taffy::Style {
                                flex_direction: taffy::FlexDirection::Row,
                                padding: taffy_tools::rect_lp(10.0, 10.0, 0.0, 0.0),
                                gap: taffy_tools::size_lp(10.0, 0.0),
                                ..Default::default()
                            })
                            .add(|tui| {
                                tui.style(taffy::Style {
                                    size: taffy::Size::from_lengths(
                                        state.images_size_vec.x,
                                        state.images_size_vec.y,
                                    ),
                                    align_self: Some(taffy::AlignItems::Center),
                                    justify_self: Some(taffy::AlignItems::Center),
                                    ..Default::default()
                                })
                                .ui(|ui| {
                                    match &image.uri {
                                        Ok(uri) => {
                                            //TODO: Image are not loaded in background.
                                            let image = egui::Image::from_uri(uri);
                                            let result = image.load_for_size(ctx, state.images_size_vec);
                                            match result {
                                                Ok(poll) => match poll {
                                                    egui::load::TexturePoll::Pending { size: _ } => {
                                                        ui.spinner()
                                                    }
                                                    egui::load::TexturePoll::Ready { texture: _ } => {
                                                        ui.add_sized(state.images_size_vec, image)
                                                    }
                                                },
                                                // TODO: store error to display in right panel
                                                // TODO: Better handling of invalid image
                                                Err(_) => ui.add(egui::Image::from_uri("invalid")),
                                            };
                                        }
                                        Err(_) => {
                                            // TODO: store error to display in right panel
                                            // TODO: Better handling of invalid image
                                            ui.add(egui::Image::from_uri("invalid"));
                                        }
                                    };
                                });
                                // TODO: draw right widget
                                tui.label(&image.name);
                            });
                            tui.separator();
                        }
                    });
            });
            self.initialized = true;
            state.store(ctx, ui.id());
        });
    }
}
