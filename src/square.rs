use crate::file::BoardFile;
use crate::piece::Piece;
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
            None => "_".to_string()
        };
        format!("{}{}: {}", self.file.render(), self.rank, p)
    }
}
