use crate::assets;
use crate::taffy_tools;
use eframe::egui;
use egui_taffy::taffy;
use egui_taffy::{self, TuiBuilderLogic};

#[derive(Default)]
pub struct WelcomePannel {
    files: String,
}

/*
impl Default for WelcomePannel {
    fn default() -> Self {
        Self {
            files: String::new(),
        }
    }
}
 */

impl eframe::App for WelcomePannel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.style_mut(|style| {
                style.wrap_mode = Some(egui::TextWrapMode::Extend);
                style.spacing.button_padding = egui::vec2(15.0, 2.0);
            });
            ctx.options_mut(|options| {
                options.max_passes = std::num::NonZeroUsize::new(15).unwrap();
            });

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
                                egui::Theme::Light => {
                                    egui::widgets::Image::new(assets::SPLASH_ON_LIGHT)
                                }
                                egui::Theme::Dark => {
                                    egui::widgets::Image::new(assets::SPLASH_ON_DARK)
                                }
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
                        tui.style(taffy::Style {
                            flex_grow: 1.,
                            ..Default::default()
                        })
                        .ui_add(
                            egui::TextEdit::singleline(&mut self.files)
                                .desired_width(f32::INFINITY),
                        );
                        tui.style(taffy::Style {
                            flex_grow: 0.,
                            ..Default::default()
                        })
                        .ui(|ui| {
                            ui.add(egui::Button::new("Select images or folders ..."));
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
                        ui.add(egui::Button::new("Open !"));
                    });
                });
        });
    }
}
