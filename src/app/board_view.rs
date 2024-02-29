use eframe::{
    egui::{Sense, Widget},
    epaint::{Color32, Rect, Rounding, Vec2},
};

use crate::board::{File, Rank, Square};

/// A view of the chess board
pub struct BoardView;

impl BoardView {
    /// Constructs a new `BoardView`
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BoardView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for BoardView {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let size = 650.0;
        let (rect, response) = ui.allocate_at_least(Vec2::new(size, size), Sense::click_and_drag());

        let painter = ui.painter();

        for file in 'a'..'h' {
            for rank in 1..8 {
                let square = Square::new(File::new(file), Rank::new(rank));
                let (f, r) = square.indices();

                let color = match (f + r) % 2 {
                    0 => Color32::from_rgb(195, 163, 113),
                    1 => Color32::from_rgb(113, 78, 47),
                    _ => unreachable!(),
                };

                let (f, r) = (f as f32 / 8.0, r as f32 / 8.0);

                let rect = Rect {
                    min: rect.min
                        + Vec2 {
                            x: f * size,
                            y: r * size,
                        },
                    max: rect.min
                        + Vec2 {
                            x: (f + 0.125) * size,
                            y: (r + 0.125) * size,
                        },
                };

                painter.rect_filled(rect, Rounding::ZERO, color);
                // draw piece images
            }
        }

        response
    }
}
