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
}
