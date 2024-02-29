//! Rust chess implementation

#![warn(missing_docs)]

use crate::app::App;

/// A chess board and associated position
pub mod board;

/// A chess move
pub mod r#move;

/// A chess piece
pub mod piece;

/// The chess GUI
pub mod app;

fn main() {
    eframe::run_native(
        "Chess",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .expect("unable to run native app");
}
