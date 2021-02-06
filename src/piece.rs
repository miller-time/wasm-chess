use crate::color::Color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }

    pub fn render(&self) -> String {
        match self.color {
            Color::Black => match self.piece_type {
                PieceType::King => "♚".to_string(),
                PieceType::Queen => "♛".to_string(),
                PieceType::Bishop => "♝".to_string(),
                PieceType::Knight => "♞".to_string(),
                PieceType::Rook => "♜".to_string(),
                PieceType::Pawn => "♟".to_string(),
            },
            Color::White => match self.piece_type {
                PieceType::King => "♔".to_string(),
                PieceType::Queen => "♕".to_string(),
                PieceType::Bishop => "♗".to_string(),
                PieceType::Knight => "♘".to_string(),
                PieceType::Rook => "♖".to_string(),
                PieceType::Pawn => "♙".to_string(),
            },
        }
    }
}
