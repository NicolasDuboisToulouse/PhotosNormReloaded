use crate::{assets, taffy_tools};
use core::assets as core_assets;
use core::metadata::Metadata;
use eframe::egui;
use egui_taffy::{
    self,
    taffy::{
        self,
        prelude::{FromLength, FromPercent},
    },
    TuiBuilderLogic,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

struct ImageData<'a> {
    #[allow(dead_code)] // TODO: May be used later
    path: &'a PathBuf,
    filename: String,
    error: Option<String>,
    uri: Option<String>,
    #[allow(dead_code)] // TODO: May be used later
    metadata: Option<Metadata>,
    camera: String,
    description: String,
    date: String,
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

trait UiTool {
    fn wrapped_label(&mut self, text: impl Into<egui::WidgetText>);
}

impl UiTool for egui::Ui {
    fn wrapped_label(&mut self, text: impl Into<egui::WidgetText>) {
        self.add(egui::Label::new(text).wrap_mode(egui::TextWrapMode::Wrap));
    }
}

impl MainPanel<'_> {
    pub fn show(image_paths: &[PathBuf]) -> eframe::Result {
        // Compute ImageData for each image
        let images = image_paths
            .iter()
            .map(|path| {
                let filename = path
                    .file_name()
                    .unwrap_or(std::ffi::OsStr::new("Unexpected invalid file name"))
                    .to_string_lossy()
                    .to_string();

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

                let metadata = match uri {
                    Ok(_) => Metadata::new(path),
                    Err(ref e) => Err(std::io::Error::other(e.to_string())),
                };

                let camera = match metadata {
                    Ok(ref m) => m.camera_info().to_string(),
                    Err(_) => String::new(),
                };

                let description = match metadata {
                    Ok(ref m) => m.description().unwrap_or(String::new()),
                    Err(_) => String::new(),
                };

                let date = match metadata {
                    Ok(ref m) => m.exif_date().unwrap_or(String::new()),
                    Err(_) => String::new(),
                };

                let error = match metadata {
                    Ok(_) => None,
                    Err(ref e) => Some(e.to_string()),
                };

                ImageData {
                    path,
                    filename,
                    error,
                    uri: uri.ok(),
                    camera,
                    metadata: metadata.ok(),
                    description,
                    date,
                }
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
                        let infos_width = tui.egui_ui().available_width() - state.images_size_vec.x - 20.0;

                        for image in &mut self.images {
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
                                        Some(uri) => {
                                            //TODO: Image are not loaded in background.
                                            let image_widget = egui::Image::from_uri(uri);
                                            let result =
                                                image_widget.load_for_size(ctx, state.images_size_vec);
                                            match result {
                                                Ok(poll) => match poll {
                                                    egui::load::TexturePoll::Pending { size: _ } => {
                                                        ui.spinner()
                                                    }
                                                    egui::load::TexturePoll::Ready { texture: _ } => {
                                                        ui.add_sized(state.images_size_vec, image_widget)
                                                    }
                                                },
                                                Err(ref e) => {
                                                    image.error = Some(e.to_string());
                                                    ui.add_sized(
                                                        state.images_size_vec,
                                                        egui::widgets::Image::new(assets::ERROR_IMG),
                                                    )
                                                }
                                            };
                                        }
                                        None => {
                                            ui.add_sized(
                                                state.images_size_vec,
                                                egui::widgets::Image::new(assets::ERROR_IMG),
                                            );
                                        }
                                    };
                                });

                                tui.style(taffy::Style {
                                    size: taffy::Size {
                                        width: taffy::Dimension::from_length(infos_width),
                                        height: taffy::Dimension::from_percent(0.0),
                                    },
                                    display: taffy::Display::Block,
                                    ..Default::default()
                                })
                                .add(|tui| {
                                    tui.ui(|ui| {
                                        egui::Grid::new(&image.filename)
                                            .num_columns(2)
                                            .spacing(egui::Vec2::new(4.0, 10.0))
                                            .show(ui, |ui| {
                                                ui.label("File:");
                                                ui.label(&image.filename);
                                                ui.end_row();

                                                match image.error {
                                                    Some(ref e) => {
                                                        ui.label("Error:");
                                                        ui.wrapped_label(e);
                                                        ui.end_row();
                                                    }
                                                    None => {
                                                        ui.label("Camera:");
                                                        ui.wrapped_label(&image.camera);
                                                        ui.end_row();

                                                        ui.label("Description: ");
                                                        ui.add(
                                                            egui::TextEdit::singleline(
                                                                &mut image.description,
                                                            )
                                                            .desired_width(f32::INFINITY),
                                                        );
                                                        ui.end_row();

                                                        ui.label("Date: ");
                                                        ui.add(
                                                            egui::TextEdit::singleline(&mut image.date)
                                                                .desired_width(f32::INFINITY),
                                                        );
                                                        ui.end_row();
                                                    }
                                                }
                                            });
                                    });
                                });
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
