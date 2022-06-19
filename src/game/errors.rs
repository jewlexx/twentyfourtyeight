use thiserror::Error as AsError;

pub(super) type Result<T> = std::result::Result<T, GameError>;

#[derive(Debug, AsError)]
pub enum GameError {
    #[error("Board Error: {0}")]
    BoardError(#[from] crate::board::errors::BoardError),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}
