use thiserror::Error as AsError;

pub(super) type Result<T> = std::result::Result<T, GameError>;

#[derive(Debug, AsError, PartialEq, Eq)]
pub enum GameError {
    #[error("Board Error: {0}")]
    BoardError(#[from] crate::board::errors::BoardError),
}
