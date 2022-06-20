use std::{error::Error, io};

use bevy::prelude::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Widget},
    Frame, Terminal,
};

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

    pub fn display(&self) -> Result<()> {
        enable_raw_mode()?;

        let mut stdout = std::io::stdout();

        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);

        let mut terminal = Terminal::new(backend)?;

        disable_raw_mode()?;

        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>) -> Result<()> {
        let layout = Layout::default().direction(Direction::Horizontal);

        let block = Block::default();
        let span = Span::from("Hello, world!");

        Ok(())
    }
}
