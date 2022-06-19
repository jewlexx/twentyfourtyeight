use crate::board::Board;

pub mod errors;

use errors::Result;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Result<Self> {
        let board = Board::gen_new()?;

        Ok(Game { board })
    }
}
