//! Rust chess implementation

#![warn(missing_docs)]

use board::{File, Rank, Square};
use r#move::Move;

use crate::piece::Piece;

/// A chess board and associated position
pub mod board;

/// A chess move
pub mod r#move;

/// A chess piece
pub mod piece;

fn main() {}
