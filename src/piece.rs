use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
	pub kind: PieceKind,
	pub color: PieceColor,
}

impl Piece {
	pub const WHITE_PAWN: Self = Piece::new(PieceKind::Pawn, PieceColor::White);
	pub const WHITE_KNIGHT: Self = Piece::new(PieceKind::Knight, PieceColor::White);
	pub const WHITE_BISHOP: Self = Piece::new(PieceKind::Bishop, PieceColor::White);
	pub const WHITE_ROOK: Self = Piece::new(PieceKind::Rook, PieceColor::White);
	pub const WHITE_QUEEN: Self = Piece::new(PieceKind::Queen, PieceColor::White);
	pub const WHITE_KING: Self = Piece::new(PieceKind::King, PieceColor::White);

	pub const BLACK_PAWN: Self = Piece::new(PieceKind::Pawn, PieceColor::Black);
	pub const BLACK_KNIGHT: Self = Piece::new(PieceKind::Knight, PieceColor::Black);
	pub const BLACK_BISHOP: Self = Piece::new(PieceKind::Bishop, PieceColor::Black);
	pub const BLACK_ROOK: Self = Piece::new(PieceKind::Rook, PieceColor::Black);
	pub const BLACK_QUEEN: Self = Piece::new(PieceKind::Queen, PieceColor::Black);
	pub const BLACK_KING: Self = Piece::new(PieceKind::King, PieceColor::Black);

	pub const fn new(kind: PieceKind, color: PieceColor) -> Self {
		Self { kind, color }
	}

	pub const fn opposite(&self) -> Self {
		Self::new(self.kind, self.color.opposite())
	}

	pub const fn is_white(&self) -> bool {
		matches!(self.color, PieceColor::White)
	}

	pub const fn is_black(&self) -> bool {
		matches!(self.color, PieceColor::Black)
	}

	pub const fn is_pawn(&self) -> bool {
		matches!(self.kind, PieceKind::Pawn)
	}

	pub const fn is_knight(&self) -> bool {
		matches!(self.kind, PieceKind::Knight)
	}

	pub const fn is_bishop(&self) -> bool {
		matches!(self.kind, PieceKind::Bishop)
	}

	pub const fn is_rook(&self) -> bool {
		matches!(self.kind, PieceKind::Rook)
	}

	pub const fn is_queen(&self) -> bool {
		matches!(self.kind, PieceKind::Queen)
	}

	pub const fn is_king(&self) -> bool {
		matches!(self.kind, PieceKind::King)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

impl Display for PieceKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PieceKind::Pawn => write!(f, ""),
			PieceKind::Knight => write!(f, "N"),
			PieceKind::Bishop => write!(f, "B"),
			PieceKind::Rook => write!(f, "R"),
			PieceKind::Queen => write!(f, "Q"),
			PieceKind::King => write!(f, "K"),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColor {
	White,
	Black,
}

impl PieceColor {
	pub const fn opposite(&self) -> Self {
		match self {
			PieceColor::White => PieceColor::Black,
			PieceColor::Black => PieceColor::White,
		}
	}
}
