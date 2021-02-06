use crate::file::BoardFile;
use crate::square::{default_squares, Square};
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
        let squares = default_squares();

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
