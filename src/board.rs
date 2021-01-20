use crate::file::BoardFile;
use crate::color::Color;
use crate::piece::{Piece, PieceType};
use crate::square::Square;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Board {
    squares: Vec<Square>,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        let mut squares = Vec::new();

        squares.push(Square::new(Some(Piece::new(PieceType::Rook, Color::White)), 1, 1));
        squares.push(Square::new(Some(Piece::new(PieceType::Knight, Color::White)), 1, 2));
        squares.push(Square::new(Some(Piece::new(PieceType::Bishop, Color::White)), 1, 3));
        squares.push(Square::new(Some(Piece::new(PieceType::Queen, Color::White)), 1, 4));
        squares.push(Square::new(Some(Piece::new(PieceType::King, Color::White)), 1, 5));
        squares.push(Square::new(Some(Piece::new(PieceType::Bishop, Color::White)), 1, 6));
        squares.push(Square::new(Some(Piece::new(PieceType::Knight, Color::White)), 1, 7));
        squares.push(Square::new(Some(Piece::new(PieceType::Rook, Color::White)), 1, 8));

        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::White)), 2, 1));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::White)), 2, 2));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::White)), 2, 3));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::White)), 2, 4));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::White)), 2, 5));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::White)), 2, 6));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::White)), 2, 7));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::White)), 2, 8));

        for rank in 3..=6 {
            for file in 1..=6 {
                let square = Square::new(None, rank, file);
                squares.push(square);
            }
        }

        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::Black)), 7, 1));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::Black)), 7, 2));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::Black)), 7, 3));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::Black)), 7, 4));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::Black)), 7, 5));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::Black)), 7, 6));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::Black)), 7, 7));
        squares.push(Square::new(Some(Piece::new(PieceType::Pawn, Color::Black)), 7, 8));

        squares.push(Square::new(Some(Piece::new(PieceType::Rook, Color::Black)), 8, 1));
        squares.push(Square::new(Some(Piece::new(PieceType::Knight, Color::Black)), 8, 2));
        squares.push(Square::new(Some(Piece::new(PieceType::Bishop, Color::Black)), 8, 3));
        squares.push(Square::new(Some(Piece::new(PieceType::Queen, Color::Black)), 8, 4));
        squares.push(Square::new(Some(Piece::new(PieceType::King, Color::Black)), 8, 5));
        squares.push(Square::new(Some(Piece::new(PieceType::Bishop, Color::Black)), 8, 6));
        squares.push(Square::new(Some(Piece::new(PieceType::Knight, Color::Black)), 8, 7));
        squares.push(Square::new(Some(Piece::new(PieceType::Rook, Color::Black)), 8, 8));

        Board { squares }
    }

    pub fn render(&self) -> String {
        self.squares
            .iter()
            .map(|square| square.render())
            .collect::<Vec<String>>()
            .join("\n")
    }

    // given 1-indexed `rank` and `file`, return the index into the `squares`
    // vector for the associated square
    fn square_index(rank: u8, file: BoardFile) -> u8 {
        (file as u8 - 1) * 8 + (rank - 1)
    }
}
