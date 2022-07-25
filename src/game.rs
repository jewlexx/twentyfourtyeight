use bevy::prelude::*;

pub mod errors;

use crate::board::Board;

use errors::Result;

#[derive(Debug, Copy, Clone, Component, PartialEq, Eq)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Result<Self> {
        let board = Board::gen_new()?;

        Ok(Game { board })
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
}
