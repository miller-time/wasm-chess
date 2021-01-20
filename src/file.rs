use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum BoardFile {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
}

impl BoardFile {
    pub fn from_u8(n: u8) -> BoardFile {
        match n {
            1 => BoardFile::A,
            2 => BoardFile::B,
            3 => BoardFile::C,
            4 => BoardFile::D,
            5 => BoardFile::E,
            6 => BoardFile::F,
            7 => BoardFile::G,
            8 => BoardFile::H,
            value => {
                panic!("{} is an invalid file", value);
            }
        }
    }

    pub fn render(&self) -> String {
        match self {
            BoardFile::A => "a".to_string(),
            BoardFile::B => "b".to_string(),
            BoardFile::C => "c".to_string(),
            BoardFile::D => "d".to_string(),
            BoardFile::E => "e".to_string(),
            BoardFile::F => "f".to_string(),
            BoardFile::G => "g".to_string(),
            BoardFile::H => "h".to_string(),
        }
    }
}
