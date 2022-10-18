use thiserror::Error as AsError;

pub(super) type Result<T> = std::result::Result<T, BoardError>;

#[derive(Debug, AsError, PartialEq, Eq)]
pub enum BoardError {
    #[error("Must be between {min} and {max}. Got {value}")]
    RangeError {
        min: usize,
        max: usize,
        value: usize,
    },

    #[error("Had error at cell {row} {column}")]
    CellError {
        row: usize,
        column: usize,
        error: CellError,
    },

    #[error("Found invalid direction {direction}")]
    InvalidDirection { direction: String },
}

#[derive(Debug, AsError, PartialEq, Eq)]
pub enum CellError {
    #[error("No value")]
    NoValue,
}
