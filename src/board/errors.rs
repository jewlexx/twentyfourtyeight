use thiserror::Error as AsError;

use super::{Axis, Direction};

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

    #[error("Failed with conversion")]
    ConversionError(#[from] std::convert::Infallible),

    #[error("Invalid direction {0} for axis {1}")]
    InvalidMoveDirection(Direction, Axis),
}

#[derive(Debug, AsError, PartialEq, Eq)]
pub enum CellError {
    #[error("No value")]
    NoValue,
}
