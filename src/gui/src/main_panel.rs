mod image;
mod logger;
use crate::{assets, taffy_tools};
use core::assets as core_assets;
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

pub struct MainPanel {
    /// list of selected paths (result)
    images: Vec<image::ImageResult>,
    show_logger: bool,
    // TODO: remove this hack
    initialized: bool,
}

//
// State saved in percistence
//
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

//
// Helpers
//
trait UiTool {
    fn wrapped_label(&mut self, text: impl Into<egui::WidgetText>);
}

impl UiTool for egui::Ui {
    fn wrapped_label(&mut self, text: impl Into<egui::WidgetText>) {
        self.add(egui::Label::new(text).wrap_mode(egui::TextWrapMode::Wrap));
    }
}

//
// Main pannel itself
//
impl MainPanel {
    /// Display the main pannel
    pub fn show(image_paths: &[PathBuf]) -> eframe::Result {
        let images = image_paths.iter().map(image::Data::new).collect::<Vec<_>>();

        let panel = MainPanel {
            images,
            show_logger: false,
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

    /// Draw the left widget for an image
    fn add_left_image(state: State, tui: &mut egui_taffy::Tui, widget: impl egui::Widget) {
        tui.style(taffy::Style {
            size: taffy::Size::from_lengths(state.images_size_vec.x, state.images_size_vec.y),
            align_self: Some(taffy::AlignItems::Center),
            justify_self: Some(taffy::AlignItems::Center),
            ..Default::default()
        })
        .ui(|ui| {
            ui.add_sized(state.images_size_vec, widget);
        });
    }

    /// Draw the right grid for an image
    fn add_right_grid<T>(
        tui: &mut egui_taffy::Tui,
        width: f32,
        filename: &str,
        content: impl FnOnce(&mut egui::Ui) -> T,
    ) {
        tui.style(taffy::Style {
            size: taffy::Size {
                width: taffy::Dimension::from_length(width),
                height: taffy::Dimension::from_percent(0.0),
            },
            display: taffy::Display::Block,
            ..Default::default()
        })
        .add(|tui| {
            tui.ui(|ui| {
                egui::Grid::new(filename)
                    .num_columns(2)
                    .spacing(egui::Vec2::new(4.0, 10.0))
                    .show(ui, |ui| {
                        ui.label("File:");
                        ui.label(filename);
                        ui.end_row();
                        content(ui);
                    });
            });
        });
    }
}

impl eframe::App for MainPanel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_logger {
            self.show_logger = logger::Logger::show(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_logger {
                ui.disable()
            };
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
                            self.show_logger = true;
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
                            self.show_logger = true;
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
                                match image {
                                    Ok(image) => {
                                        //TODO: Image are not loaded in background.
                                        let image_widget = egui::Image::from_uri(&image.uri);
                                        let result = image_widget.load_for_size(ctx, state.images_size_vec);
                                        match result {
                                            Ok(poll) => match poll {
                                                egui::load::TexturePoll::Pending { size: _ } => {
                                                    Self::add_left_image(state, tui, egui::Spinner::new());
                                                }
                                                egui::load::TexturePoll::Ready { texture: _ } => {
                                                    Self::add_left_image(state, tui, image_widget)
                                                }
                                            },
                                            Err(_) => {
                                                Self::add_left_image(
                                                    state,
                                                    tui,
                                                    egui::widgets::Image::new(assets::ERROR_IMG),
                                                );
                                            }
                                        };
                                        Self::add_right_grid(tui, infos_width, &image.filename, |ui| {
                                            ui.label("Camera:");
                                            ui.wrapped_label(&image.camera);
                                            ui.end_row();

                                            ui.label("Description: ");
                                            ui.add(
                                                egui::TextEdit::singleline(&mut image.description)
                                                    .desired_width(f32::INFINITY),
                                            );
                                            ui.end_row();

                                            // TODO: better date/time widget
                                            ui.label("Date: ");
                                            ui.add(
                                                egui::TextEdit::singleline(&mut image.date)
                                                    .desired_width(f32::INFINITY),
                                            );
                                            ui.end_row();
                                        });
                                    }

                                    Err(error) => {
                                        Self::add_left_image(
                                            state,
                                            tui,
                                            egui::widgets::Image::new(assets::ERROR_IMG),
                                        );
                                        Self::add_right_grid(tui, infos_width, &error.filename, |ui| {
                                            ui.label("Error:");
                                            ui.wrapped_label(error.to_string());
                                            ui.end_row();
                                        });
                                    }
                                };
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
