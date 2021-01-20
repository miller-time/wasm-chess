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
