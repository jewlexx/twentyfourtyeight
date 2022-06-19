use crate::board::Board;

pub mod errors;

use errors::Result;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Widget},
    Frame, Terminal,
};

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

    fn ui<B: Backend>(&self, f: &mut Frame<B>) -> Result<()> {
        let layout = Layout::default().direction(Direction::Horizontal).conta

        let block = Block::default();
        let span = Span::from("Hello, world!");

        Ok(())
    }
}
