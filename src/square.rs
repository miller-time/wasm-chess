use crate::color::Color;
use crate::file::BoardFile;
use crate::piece::{Piece, PieceType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Square {
    piece: Option<Piece>,
    rank: u8,
    file: BoardFile,
}

impl Square {
    pub fn new(piece: Option<Piece>, rank: u8, file: u8) -> Square {
        Square {
            piece,
            rank,
            file: BoardFile::from_u8(file),
        }
    }

    pub fn render(&self) -> String {
        let p = match &self.piece {
            Some(piece) => piece.render(),
            None => "_".to_string(),
        };
        format!("{}{}: {}", self.file.render(), self.rank, p)
    }
}

pub fn default_squares() -> Vec<Square> {
    let mut squares = Vec::new();

    let white_rook = Piece::new(PieceType::Rook, Color::White);
    squares.push(Square::new(Some(white_rook), 1, 1));

    let white_knight = Piece::new(PieceType::Knight, Color::White);
    squares.push(Square::new(Some(white_knight), 1, 2));

    let white_bishop = Piece::new(PieceType::Bishop, Color::White);
    squares.push(Square::new(Some(white_bishop), 1, 3));

    let white_queen = Piece::new(PieceType::Queen, Color::White);
    squares.push(Square::new(Some(white_queen), 1, 4));

    let white_king = Piece::new(PieceType::King, Color::White);
    squares.push(Square::new(Some(white_king), 1, 5));

    let white_bishop = Piece::new(PieceType::Bishop, Color::White);
    squares.push(Square::new(Some(white_bishop), 1, 6));

    let white_knight = Piece::new(PieceType::Knight, Color::White);
    squares.push(Square::new(Some(white_knight), 1, 7));

    let white_rook = Piece::new(PieceType::Rook, Color::White);
    squares.push(Square::new(Some(white_rook), 1, 8));

    for file in 1..=8 {
        let pawn = Piece::new(PieceType::Pawn, Color::White);
        squares.push(Square::new(Some(pawn), 2, file));
    }

    for rank in 3..=6 {
        for file in 1..=8 {
            squares.push(Square::new(None, rank, file));
        }
    }

    for file in 1..=8 {
        let pawn = Piece::new(PieceType::Pawn, Color::Black);
        squares.push(Square::new(Some(pawn), 7, file));
    }

    let black_rook = Piece::new(PieceType::Rook, Color::Black);
    squares.push(Square::new(Some(black_rook), 8, 1));

    let black_knight = Piece::new(PieceType::Knight, Color::Black);
    squares.push(Square::new(Some(black_knight), 8, 2));

    let black_bishop = Piece::new(PieceType::Bishop, Color::Black);
    squares.push(Square::new(Some(black_bishop), 8, 3));

    let black_queen = Piece::new(PieceType::Queen, Color::Black);
    squares.push(Square::new(Some(black_queen), 8, 4));

    let black_king = Piece::new(PieceType::King, Color::Black);
    squares.push(Square::new(Some(black_king), 8, 5));

    let black_bishop = Piece::new(PieceType::Bishop, Color::Black);
    squares.push(Square::new(Some(black_bishop), 8, 6));

    let black_knight = Piece::new(PieceType::Knight, Color::Black);
    squares.push(Square::new(Some(black_knight), 8, 7));

    let black_rook = Piece::new(PieceType::Rook, Color::Black);
    squares.push(Square::new(Some(black_rook), 8, 8));

    assert_eq!(squares.len(), 64);

    squares
}
