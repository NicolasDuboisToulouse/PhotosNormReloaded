use eframe::egui;
use egui_taffy::taffy;
use egui_taffy::TuiBuilderLogic;

pub struct Logger {
    visible: bool,
}

impl Logger {
    // Return false when the logger shall be closed
    pub fn show(ctx: &egui::Context) -> bool {
        let mut logger = Logger { visible: true };

        // Allows to center viewport on the main window
        let main_frame_rect = ctx
            .viewport(|t| t.input.viewport().inner_rect)
            .unwrap_or(egui::Rect::ZERO);

        // TODO: find a way to popup log window when main window clicked.
        // TODO: .with_always_on_top() is not a good option
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("logger"),
            egui::ViewportBuilder::default()
                .with_title("Updating images")
                .with_position(main_frame_rect.min)
                .with_inner_size(main_frame_rect.size()),
            |ctx, class| {
                assert!(
                    class != egui::ViewportClass::Embedded,
                    "This egui backend doesn't support multiple viewports",
                );
                egui::TopBottomPanel::bottom("close_panel")
                    .show_separator_line(false)
                    .show(ctx, |ui| {
                        egui_taffy::tui(ui, ui.id().with("buttom_button"))
                            .reserve_available_space()
                            .style(taffy::Style {
                                display: taffy::Display::Block,
                                ..Default::default()
                            })
                            .show(|tui| {
                                tui.add(|tui| {
                                    tui.style(taffy::Style {
                                        margin: taffy::Rect {
                                            top: taffy::LengthPercentageAuto::Length(10.0),
                                            bottom: taffy::LengthPercentageAuto::Length(0.0),
                                            left: taffy::LengthPercentageAuto::Auto,
                                            right: taffy::LengthPercentageAuto::Auto,
                                        },
                                        ..Default::default()
                                    })
                                    .ui(|ui| {
                                        if ui.button("Close").clicked() {
                                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                                        }
                                    });
                                });
                            });
                    });
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            egui::Label::new(
                                "long content long content long content long content\
                             long content long content long content long content\
                             long content long content long content long content\
                             long content long content long content\n\
                             Log content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\nLog content\nLog content\n\
                             Log content\nLog content\n",
                            )
                            .wrap(),
                        );
                    });
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    logger.visible = false;
                }
            },
        );
        logger.visible
    }
}
