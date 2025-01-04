use crate::assets;
use crate::taffy_tools;
use core::assets as core_assets;
use eframe::egui;
use eframe::egui::Widget;
use egui_taffy::taffy;
use egui_taffy::{self, TuiBuilderLogic};
use std::path::PathBuf;

pub struct WelcomePanel<'a> {
    /// list of selected paths (result)
    paths_vec: &'a mut Vec<PathBuf>,
    /// Semicolon-separated path list displayed in main text box
    paths_semicolon: String,
    /// Has user manually modified paths_semicolon ?
    user_modified: bool,
    /// Has user clicked on Open button ? (window not closed by another way)
    user_close: bool,
}

impl WelcomePanel<'_> {
    ///
    /// Display the dialog and return the list of selected paths
    /// Return an empty Vec on user cancel or error
    ///
    pub fn show() -> Vec<PathBuf> {
        let mut paths_vec: Vec<PathBuf> = Vec::new();

        let panel = WelcomePanel {
            paths_semicolon: String::new(),
            paths_vec: &mut paths_vec,
            user_modified: false,
            user_close: false,
        };

        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                icon: assets::load_app_icon(),
                ..Default::default()
            },
            ..Default::default()
        };

        // Ignore error: show() will return an empty Vec
        let _ = eframe::run_native(
            core_assets::PROJECT_NAME,
            native_options,
            Box::new(|cc| {
                //            cc.egui_ctx.set_theme(egui::ThemePreference::Light);
                egui_extras::install_image_loaders(&cc.egui_ctx);
                assets::init_egui_ctx(&cc.egui_ctx);
                Ok(Box::new(panel))
            }),
        );

        paths_vec
    }

    /// Set paths from "browse..." button
    fn set_paths_vec(&mut self, paths: Vec<PathBuf>) {
        // This conversion may break non-UTF8 paths.
        // So we keep the good result in self.paths_vec unless user modify paths_semicolon.
        self.paths_semicolon = paths
            .iter()
            .map(|p| p.to_string_lossy())
            .collect::<Vec<_>>()
            .join(";");
        *self.paths_vec = paths;
        self.user_modified = false;
    }

    /// Close has been requested
    /// On cancel -> clear result (self.paths_vec)
    /// On OK -> convert paths_semicolon to paths_vec if needed and check paths
    fn query_close(&mut self) -> bool {
        if self.user_close {
            if self.user_modified {
                // (Re)build the list of paths from the user string
                *self.paths_vec = self
                    .paths_semicolon
                    .split(";")
                    .map(PathBuf::from)
                    .collect::<Vec<_>>();
            }
            for path in &mut *self.paths_vec {
                if !path.exists() {
                    rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Error)
                        .set_title("Path not found")
                        .set_description(format!("Path not found : {}", path.display()))
                        .set_buttons(rfd::MessageButtons::Ok)
                        .show();
                    // TODO: dialog
                    self.user_close = false;
                    return false;
                }
            }
            true
        } else {
            self.paths_vec.clear();
            true
        }
    }
}

impl eframe::App for WelcomePanel<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Main widget, fill wall space
            egui_taffy::tui(ui, ui.id().with("welcome"))
                .reserve_available_space()
                .style(taffy::Style {
                    flex_direction: taffy::FlexDirection::Column,
                    padding: taffy_tools::h_percent(0.02),
                    gap: taffy::prelude::length(10.),
                    size: taffy::prelude::percent(1.),
                    ..Default::default()
                })
                .show(|tui| {
                    if ctx.input(|i| i.viewport().close_requested()) && !self.query_close() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    }

                    // Splash image, 60% height
                    tui.style(taffy::Style {
                        flex_grow: 0.,
                        size: taffy::Size {
                            width: taffy::Dimension::Auto,
                            height: taffy::Dimension::Percent(0.6),
                        },
                        padding: taffy_tools::rect_lp(0.0, 10.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .ui(|ui| {
                        ui.centered_and_justified(|ui| {
                            ui.add(match ctx.theme() {
                                egui::Theme::Light => egui::widgets::Image::new(assets::SPLASH_ON_LIGHT),
                                egui::Theme::Dark => egui::widgets::Image::new(assets::SPLASH_ON_DARK),
                            });
                        });
                    });

                    // Browser
                    tui.style(taffy::Style {
                        flex_grow: 0.,
                        flex_direction: taffy::FlexDirection::Row,
                        gap: taffy::prelude::length(10.),
                        padding: taffy_tools::h_percent(0.1),
                        align_items: Some(taffy::AlignItems::Start),
                        ..Default::default()
                    })
                    .add(|tui| {
                        let paths_responce = tui
                            .style(taffy::Style {
                                flex_grow: 1.,
                                ..Default::default()
                            })
                            .ui_add(
                                egui::TextEdit::singleline(&mut self.paths_semicolon)
                                    .desired_width(f32::INFINITY),
                            );
                        if paths_responce.changed() {
                            self.user_modified = true;
                        }
                        tui.style(taffy::Style {
                            flex_grow: 0.,
                            ..Default::default()
                        })
                        .ui(|ui| {
                            let button = egui::Button::new("folders...");
                            if button
                                .ui(ui)
                                .on_hover_ui(|ui| {
                                    ui.label("Load all images within folder(s) (non-recursive)");
                                })
                                .clicked()
                            {
                                if let Some(paths) = rfd::FileDialog::new().pick_folders() {
                                    self.set_paths_vec(paths);
                                }
                            }
                        });
                        tui.style(taffy::Style {
                            flex_grow: 0.,
                            ..Default::default()
                        })
                        .ui(|ui| {
                            let button = egui::Button::new("images...");
                            if button
                                .ui(ui)
                                .on_hover_ui(|ui| {
                                    ui.label("Load individual image(s)");
                                })
                                .clicked()
                            {
                                if let Some(paths) = rfd::FileDialog::new().pick_files() {
                                    self.set_paths_vec(paths);
                                }
                            }
                        });
                    });

                    // Gap
                    tui.style(taffy::Style {
                        flex_grow: 1.,
                        ..Default::default()
                    })
                    .add_empty();

                    // Action buttons
                    tui.style(taffy::Style {
                        flex_grow: 0.,
                        align_self: Some(taffy::AlignItems::Center),
                        padding: taffy_tools::rect_lp(0.0, 10.0, 0.0, 0.0),
                        ..Default::default()
                    })
                    .ui(|ui| {
                        let button = egui::Button::new("Open !");
                        if ui
                            .add_enabled(!self.paths_semicolon.is_empty(), button)
                            .on_hover_ui(|ui| {
                                ui.label("Just load images! Do not modify them yet!");
                            })
                            .on_disabled_hover_ui(|ui| {
                                ui.label("Just load images! Do not modify them yet!");
                            })
                            .clicked()
                        {
                            self.user_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        });
    }
}
