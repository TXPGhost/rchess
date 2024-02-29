use std::fmt::{Debug, Display};

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
    pub fn new_init() -> Self {
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
            }
            | Move::PromotionCapture {
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
            Move::EnPassantCapture {
                capturing,
                from,
                to,
            } => {
                let pawn = self.take_piece(to).expect("illegal move");
                self.set_piece(from, pawn);
                self.set_piece(Square::new(to.file, from.rank), capturing);
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
        Self::new_init()
    }
}

/// A rank on the chess board, in the range (1..=8)
#[derive(Clone, Copy, Debug)]
pub struct Rank(u8);

impl Rank {
    /// Constructs a new rank from the given integer
    #[inline]
    pub fn new(rank: u8) -> Self {
        assert!((1..=8).contains(&rank), "illegal rank");
        Self(rank - 1)
    }

    /// Iterates through every rank from 1 to 8
    pub fn for_each(mut action: impl FnMut(Self)) {
        for rank in 1..8 {
            action(Self::new(rank));
        }
    }

    /// Returns the index associated with this rank
    pub fn index(&self) -> u8 {
        self.0
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

/// A file on the chess board, in the range ('a'..='f')
#[derive(Clone, Copy, Debug)]
pub struct File(u8);

impl File {
    /// Constructs a new file from the given lowercase character
    #[inline]
    pub fn new(file: char) -> Self {
        assert!(('a'..='h').contains(&file), "illegal file");
        Self((file as u32 - 'a' as u32) as u8)
    }

    /// Iterates through every file from 'a' to 'f'
    pub fn for_each(mut action: impl FnMut(Self)) {
        for file in 'a'..'h' {
            action(Self::new(file));
        }
    }

    /// Returns the index associated with this file
    pub fn index(&self) -> u8 {
        self.0
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.0 as u32 + 'a' as u32) as u8 as char)
    }
}

impl TryFrom<&str> for File {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a" | "A" => Ok(Self::new('a')),
            "b" | "B" => Ok(Self::new('b')),
            "c" | "C" => Ok(Self::new('c')),
            "d" | "D" => Ok(Self::new('d')),
            "e" | "E" => Ok(Self::new('e')),
            "f" | "F" => Ok(Self::new('f')),
            "g" | "G" => Ok(Self::new('g')),
            "h" | "H" => Ok(Self::new('h')),
            _ => Err("illegal file"),
        }
    }
}

/// A square on the chess board, i.e. "f3"
#[derive(Clone, Copy, Debug)]
pub struct Square {
    /// The file associated with this square
    pub file: File,

    /// The rank associated with this square
    pub rank: Rank,
}

impl Square {
    /// Constructs a new square from the given rank and file
    pub fn new<F: TryInto<File>, R: TryInto<Rank>>(file: F, rank: R) -> Self
    where
        <F as std::convert::TryInto<File>>::Error: std::fmt::Debug,
        <R as std::convert::TryInto<Rank>>::Error: std::fmt::Debug,
    {
        Self {
            file: file.try_into().unwrap(),
            rank: rank.try_into().unwrap(),
        }
    }

    /// Returns the indices associated with this square (rank and file)
    pub fn indices(&self) -> (u8, u8) {
        (self.file.index(), self.rank.index())
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl From<&str> for Square {
    fn from(value: &str) -> Self {
        assert!(value.len() == 2, "input string must be of length 2");
        let mut chars = value.chars();
        // TODO: optimize this?
        Self::new(
            chars.next().unwrap().to_string().as_ref(),
            chars.next().unwrap().to_string().as_ref(),
        )
    }
}
