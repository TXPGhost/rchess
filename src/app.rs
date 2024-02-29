use eframe::egui;

use self::board_view::BoardView;

/// A view of the chess board
pub mod board_view;

/// The chess user interface
pub struct App {
    ui_scale: f32,
}

impl App {
    /// Creates a new chess app
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { ui_scale: 1.0 }
    }
}

impl eframe::App for App {
    #[allow(clippy::needless_if)]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(self.ui_scale);
        egui::TopBottomPanel::top("MenuBar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Game", |ui| {
                    if ui.button("New").clicked() {}

                    ui.separator();

                    if ui.button("Save").clicked() {}
                    if ui.button("Load").clicked() {}
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Flip board").clicked() {}
                    if ui.button("Automatically flip board").clicked() {}
                    if ui.button("Show coordinates").clicked() {}
                });
                ui.menu_button("Rules", |ui| {
                    if ui.button("Allow editing the past").clicked() {}
                    if ui.button("Edit clock settings").clicked() {}
                    // todo
                });

                // TODO: add fuzzy finder
            });
            ui.add(BoardView::new());
        });
        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
