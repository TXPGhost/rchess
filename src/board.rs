use std::fmt::Display;

use crate::{
	piece::{Piece, PieceKind},
	r#move::Move,
};

/// The current state of a chess board
pub struct Board {
	pieces: [[Option<Piece>; 8]; 8],
}

impl Board {
	/// Creates a new chess board with the standard starting position
	pub fn new() -> Self {
		let mut pieces = [[None; 8]; 8];

		pieces[0][0] = Some(Piece::WHITE_ROOK);
		pieces[0][1] = Some(Piece::WHITE_KNIGHT);
		pieces[0][2] = Some(Piece::WHITE_BISHOP);
		pieces[0][3] = Some(Piece::WHITE_QUEEN);
		pieces[0][4] = Some(Piece::WHITE_KING);
		pieces[0][5] = Some(Piece::WHITE_BISHOP);
		pieces[0][6] = Some(Piece::WHITE_KNIGHT);
		pieces[0][7] = Some(Piece::WHITE_ROOK);

		pieces[7][0] = Some(Piece::BLACK_ROOK);
		pieces[7][1] = Some(Piece::BLACK_KNIGHT);
		pieces[7][2] = Some(Piece::BLACK_BISHOP);
		pieces[7][3] = Some(Piece::BLACK_QUEEN);
		pieces[7][4] = Some(Piece::BLACK_KING);
		pieces[7][5] = Some(Piece::BLACK_BISHOP);
		pieces[7][6] = Some(Piece::BLACK_KNIGHT);
		pieces[7][7] = Some(Piece::BLACK_ROOK);

		for f in 0..8 {
			pieces[1][f] = Some(Piece::WHITE_PAWN);
			pieces[6][f] = Some(Piece::BLACK_PAWN);
		}

		Self { pieces }
	}

	/// Returns the piece at the given square
	#[inline]
	pub fn get_piece(&self, square: impl Into<Square>) -> Option<Piece> {
		let square: Square = square.into();
		self.pieces[square.rank.0 as usize][square.file.0 as usize]
	}

	/// Sets the piece at the given square
	#[inline]
	pub fn set_piece(&mut self, square: impl Into<Square>, piece: Piece) {
		let square: Square = square.into();
		self.pieces[square.rank.0 as usize][square.file.0 as usize] = Some(piece);
	}

	/// Sets the piece at the given square to be `None`
	#[inline]
	pub fn remove_piece(&mut self, square: impl Into<Square>) {
		let square: Square = square.into();
		self.pieces[square.rank.0 as usize][square.file.0 as usize] = None;
	}

	/// Takes the piece at the given square, replacing it with `None`
	#[inline]
	#[must_use]
	pub fn take_piece(&mut self, square: impl Into<Square>) -> Option<Piece> {
		let square: Square = square.into();
		self.pieces[square.rank.0 as usize][square.file.0 as usize].take()
	}

	/// Plays the given move
	pub fn play_move(&mut self, r#move: Move) {
		match r#move {
			Move::Move { from, to, .. } => {
				let piece = self.take_piece(from).expect("illegal move");
				self.set_piece(to, piece);
			}
			Move::Capture { from, to, .. } => {
				let piece = self.take_piece(from).expect("illegal capture");
				self.set_piece(to, piece);
			}
			Move::EnPassantCapture { from, to, .. } => {
				let pawn = self.take_piece(from).expect("illegal en passant capture");
				self.set_piece(to, pawn);
				self.remove_piece(Square::new(to.file, from.rank));
			}
			Move::PromotionMove {
				promoting: piece,
				from,
				to,
			} => {
				self.remove_piece(from);
				self.set_piece(to, piece);
			}
			Move::PromotionCapture {
				promoting: piece,
				from,
				to,
				..
			} => {
				self.remove_piece(from);
				self.set_piece(to, piece);
			}
			Move::KingsideCastleWhite => {
				let king = self
					.take_piece("e1")
					.expect("illegal white kingside castle");
				let rook = self
					.take_piece("h1")
					.expect("illegal white kingside castle");
				self.set_piece("g1", king);
				self.set_piece("f1", rook);
			}
			Move::QueensideCastleWhite => {
				let king = self
					.take_piece("e1")
					.expect("illegal white queenside castle");
				let rook = self
					.take_piece("a1")
					.expect("illegal white queenside castle");
				self.set_piece("c1", king);
				self.set_piece("d1", rook);
			}
			Move::KingsideCastleBlack => {
				let king = self
					.take_piece("e8")
					.expect("illegal black kingside castle");
				let rook = self
					.take_piece("h8")
					.expect("illegal black kingside castle");
				self.set_piece("g8", king);
				self.set_piece("f8", rook);
			}
			Move::QueensideCastleBlack => {
				let king = self
					.take_piece("e8")
					.expect("illegal black queenside castle");
				let rook = self
					.take_piece("a8")
					.expect("illegal black queenside castle");
				self.set_piece("c8", king);
				self.set_piece("d8", rook);
			}
		}
	}

	/// Undoes the given move
	pub fn undo_move(&mut self, r#move: Move) {
		match r#move {
			Move::Move { from, to, .. } => {
				let piece = self.take_piece(to).expect("illegal move");
				self.set_piece(from, piece);
			}
			Move::Capture {
				capturing: captured,
				from,
				to,
				..
			} => {
				let piece = self.take_piece(to).expect("illegal move");
				self.set_piece(from, piece);
				self.set_piece(to, captured);
			}
			Move::EnPassantCapture { from, to, .. } => {
				let piece = self.take_piece(to).expect("illegal move");
				self.set_piece(from, piece);
				self.set_piece(Square::new(to.file, from.rank), piece.opposite());
			}
			Move::PromotionMove {
				promoting: piece,
				from,
				to,
			} => {
				self.remove_piece(to);
				self.set_piece(from, Piece::new(PieceKind::Pawn, piece.color));
			}
			Move::PromotionCapture {
				promoting: piece,
				capturing: captures,
				from,
				to,
			} => {
				self.set_piece(to, captures);
				self.set_piece(from, Piece::new(PieceKind::Pawn, piece.color));
			}
			Move::KingsideCastleWhite => todo!(),
			Move::QueensideCastleWhite => todo!(),
			Move::KingsideCastleBlack => todo!(),
			Move::QueensideCastleBlack => todo!(),
		}
	}
}

impl Default for Board {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Rank(u8);

impl Rank {
	#[inline]
	pub fn new(rank: u8) -> Self {
		if !(1..=8).contains(&rank) {
			panic!("illegal rank");
		}
		Self(rank - 1)
	}
}

impl Display for Rank {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0 + 1)
	}
}

impl From<&str> for Rank {
	fn from(value: &str) -> Self {
		match value {
			"1" => Self::new(1),
			"2" => Self::new(2),
			"3" => Self::new(3),
			"4" => Self::new(4),
			"5" => Self::new(5),
			"6" => Self::new(6),
			"7" => Self::new(7),
			"8" => Self::new(8),
			_ => panic!("illegal rank"),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct File(u8);

impl File {
	#[inline]
	pub fn new(file: char) -> Self {
		if !('a'..='h').contains(&file) {
			panic!("illegal file")
		}
		Self((file as u32 - 'a' as u32) as u8)
	}
}

impl Display for File {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", (self.0 as u32 + 'a' as u32) as u8 as char)
	}
}

impl From<&str> for File {
	fn from(value: &str) -> Self {
		match value {
			"a" | "A" => Self::new('a'),
			"b" | "B" => Self::new('b'),
			"c" | "C" => Self::new('c'),
			"d" | "D" => Self::new('d'),
			"e" | "E" => Self::new('e'),
			"f" | "F" => Self::new('f'),
			"g" | "G" => Self::new('g'),
			"h" | "H" => Self::new('h'),
			_ => panic!("illegal file"),
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Square {
	pub file: File,
	pub rank: Rank,
}

impl Square {
	pub fn new(file: impl Into<File>, rank: impl Into<Rank>) -> Self {
		Self {
			file: file.into(),
			rank: rank.into(),
		}
	}
}

impl Display for Square {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.file, self.rank)
	}
}

impl From<&str> for Square {
	fn from(value: &str) -> Self {
		if (value.len()) != 2 {
			panic!("illegal square");
		}
		let mut chars = value.chars();
		// TODO: optimize this?
		Self::new(
			chars.next().unwrap().to_string().as_ref(),
			chars.next().unwrap().to_string().as_ref(),
		)
	}
}
