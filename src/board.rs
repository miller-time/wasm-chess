use crate::file::BoardFile;
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
        for rank in 1..=8 {
            for file in 1..=8 {
                let square = Square::new(None, rank, file);
                squares.push(square);
            }
        }
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
