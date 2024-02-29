use std::fmt::Display;

/// A chess piece with an associated kind and color
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    /// The piece's kind
    pub kind: PieceKind,

    /// The piece's color
    pub color: PieceColor,
}

impl Piece {
    /// A white pawn
    pub const WHITE_PAWN: Self = Self::new(PieceKind::Pawn, PieceColor::White);

    /// A white knight
    pub const WHITE_KNIGHT: Self = Self::new(PieceKind::Knight, PieceColor::White);

    /// A white bishop
    pub const WHITE_BISHOP: Self = Self::new(PieceKind::Bishop, PieceColor::White);

    /// A white rook
    pub const WHITE_ROOK: Self = Self::new(PieceKind::Rook, PieceColor::White);

    /// A white queen
    pub const WHITE_QUEEN: Self = Self::new(PieceKind::Queen, PieceColor::White);

    /// A white king
    pub const WHITE_KING: Self = Self::new(PieceKind::King, PieceColor::White);

    /// A black pawn
    pub const BLACK_PAWN: Self = Self::new(PieceKind::Pawn, PieceColor::Black);

    /// A black knight
    pub const BLACK_KNIGHT: Self = Self::new(PieceKind::Knight, PieceColor::Black);

    /// A black bishop
    pub const BLACK_BISHOP: Self = Self::new(PieceKind::Bishop, PieceColor::Black);

    /// A black rook
    pub const BLACK_ROOK: Self = Self::new(PieceKind::Rook, PieceColor::Black);

    /// A black queen
    pub const BLACK_QUEEN: Self = Self::new(PieceKind::Queen, PieceColor::Black);

    /// A black king
    pub const BLACK_KING: Self = Self::new(PieceKind::King, PieceColor::Black);

    /// Constructs a new chess piece with the given kind and color
    pub const fn new(kind: PieceKind, color: PieceColor) -> Self {
        Self { kind, color }
    }

    /// Returns a new piece with the opposite color
    #[must_use]
    pub const fn opposite(&self) -> Self {
        Self::new(self.kind, self.color.opposite())
    }

    /// Returns true if the given piece is white
    pub const fn is_white(&self) -> bool {
        matches!(self.color, PieceColor::White)
    }

    /// Returns true if the given piece is black
    pub const fn is_black(&self) -> bool {
        matches!(self.color, PieceColor::Black)
    }

    /// Returns true if the given piece is a pawn
    pub const fn is_pawn(&self) -> bool {
        matches!(self.kind, PieceKind::Pawn)
    }

    /// Returns true if the given piece is a knight
    pub const fn is_knight(&self) -> bool {
        matches!(self.kind, PieceKind::Knight)
    }

    /// Returns true if the given piece is a bishop
    pub const fn is_bishop(&self) -> bool {
        matches!(self.kind, PieceKind::Bishop)
    }

    /// Returns true if the given piece is a rook
    pub const fn is_rook(&self) -> bool {
        matches!(self.kind, PieceKind::Rook)
    }

    /// Returns true if the given piece is a queen
    pub const fn is_queen(&self) -> bool {
        matches!(self.kind, PieceKind::Queen)
    }

    /// Returns true if the given piece is a king
    pub const fn is_king(&self) -> bool {
        matches!(self.kind, PieceKind::King)
    }
}

/// A kind of chess piece
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    /// A pawn
    Pawn,

    /// A knight
    Knight,

    /// A bishop
    Bishop,

    /// A rook
    Rook,

    /// A queen
    Queen,

    /// A king
    King,
}

impl Display for PieceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn => write!(f, ""),
            Self::Knight => write!(f, "N"),
            Self::Bishop => write!(f, "B"),
            Self::Rook => write!(f, "R"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
        }
    }
}

/// A color of chess piece
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColor {
    /// A white piece
    White,

    /// A black piece
    Black,
}

impl PieceColor {
    /// Returns the opposite of the given color
    #[must_use]
    pub const fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
