use crate::{board::Square, piece::Piece};

/// A move on the chess board
#[derive(Clone, Copy, Debug)]
pub enum Move {
    /// A piece moving with no capture
    Move {
        /// The square the piece is moving from
        from: Square,

        /// The square the piece is moving to
        to: Square,
    },

    /// A piece capturing another piece
    Capture {
        /// The piece that is being captured
        capturing: Piece,

        /// The square the piece is moving from
        from: Square,

        /// The square the piece is moving to
        to: Square,
    },

    /// A pawn promoting to another piece by moving
    PromotionMove {
        /// The piece that the pawn is being promoted to
        promoting: Piece,

        /// The square the pawn is moving from
        from: Square,

        /// The square the pawn is moving to
        to: Square,
    },

    /// A pawn promoting to another piece by capturing
    PromotionCapture {
        /// The piece that the pawn is being promoted to
        promoting: Piece,

        /// The piece that is being captured
        capturing: Piece,

        /// The square the pawn is moving from
        from: Square,

        /// The square the pawn is moving to
        to: Square,
    },

    /// A pawn capturing an adjacent pawn via the en passant rule
    EnPassantCapture {
        /// The piece that is being captured
        capturing: Piece,

        /// The square the pawn is moving from
        from: Square,

        /// The square the pawn is moving to
        to: Square,
    },

    /// White castling kingside
    KingsideCastleWhite,

    /// White castling queenside
    QueensideCastleWhite,

    /// Black castling kingside
    KingsideCastleBlack,

    /// Black castling queenside
    QueensideCastleBlack,
}
