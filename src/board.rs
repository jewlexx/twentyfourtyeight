mod cell;
mod errors;

pub use cell::*;
use errors::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [Cell; 9],
}

impl Board {
    pub const fn new(cells: [Cell; 9]) -> Self {
        Board { cells }
    }

    pub const fn empty() -> Self {
        Board::new([Cell::Empty; 9])
    }

    pub const fn get_cells(&self) -> &[Cell; 9] {
        &self.cells
    }

    pub fn get_row(&self, row: usize) -> Result<[Cell; 3]> {
        if !(1..=3).contains(&row) {
            Err(BoardError::RangeError {
                min: 1,
                max: 3,
                value: row,
            })
        } else {
            // Forces the array to be sized
            let row = &self.cells[(row - 1) * 3..row * 3];
            Ok([row[0], row[1], row[2]])
        }
    }

    pub fn get_column(&self, column: usize) -> Result<[Cell; 3]> {
        if !(1..=3).contains(&column) {
            Err(BoardError::RangeError {
                min: 1,
                max: 3,
                value: column,
            })
        } else {
            Ok([
                self.cells[column - 1],
                self.cells[column + 2],
                self.cells[column + 5],
            ])
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Result<Cell> {
        if !(1..=3).contains(&row) {
            Err(BoardError::RangeError {
                min: 1,
                max: 3,
                value: row,
            })
        } else if !(1..=3).contains(&column) {
            Err(BoardError::RangeError {
                min: 1,
                max: 3,
                value: column,
            })
        } else {
            Ok(self.cells[(row - 1) * 3 + column - 1])
        }
    }
}

mod tests {
    use super::*;

    // Rust incorrectly warns about dead code in the following two declarations
    #[allow(dead_code)]
    const CELLS: [Cell; 9] = [
        Cell::Filled(1),
        Cell::Filled(2),
        Cell::Filled(3),
        Cell::Filled(4),
        Cell::Filled(5),
        Cell::Filled(6),
        Cell::Filled(7),
        Cell::Filled(8),
        Cell::Filled(9),
    ];

    #[allow(dead_code)]
    static BOARD: Board = Board { cells: CELLS };

    #[test]
    fn test_new() {
        let board = Board::new(CELLS);

        assert_eq!(board.get_cells(), &CELLS);
    }

    #[test]
    fn test_empty() {
        let board = Board::empty();

        assert_eq!(board.get_cells(), &[Cell::Empty; 9]);
    }

    #[test]
    fn test_get_row() {
        assert_eq!(
            BOARD.get_row(1),
            Ok([Cell::Filled(1), Cell::Filled(2), Cell::Filled(3)])
        );
        assert_eq!(
            BOARD.get_row(2),
            Ok([Cell::Filled(4), Cell::Filled(5), Cell::Filled(6)])
        );
        assert_eq!(
            BOARD.get_row(3),
            Ok([Cell::Filled(7), Cell::Filled(8), Cell::Filled(9)])
        );
        assert_eq!(
            BOARD.get_row(4),
            Err(BoardError::RangeError {
                min: 1,
                max: 3,
                value: 4,
            })
        );
    }

    #[test]
    fn test_get_column() {
        assert_eq!(
            BOARD.get_column(1),
            Ok([Cell::Filled(1), Cell::Filled(4), Cell::Filled(7)])
        );
        assert_eq!(
            BOARD.get_column(2),
            Ok([Cell::Filled(2), Cell::Filled(5), Cell::Filled(8)])
        );
        assert_eq!(
            BOARD.get_column(3),
            Ok([Cell::Filled(3), Cell::Filled(6), Cell::Filled(9)])
        );
        assert_eq!(
            BOARD.get_column(4),
            Err(BoardError::RangeError {
                min: 1,
                max: 3,
                value: 4,
            })
        );
    }

    #[test]
    fn test_get_cell() {
        assert_eq!(BOARD.get_cell(1, 1), Ok(Cell::Filled(1)));
        assert_eq!(BOARD.get_cell(2, 2), Ok(Cell::Filled(5)));
        assert_eq!(BOARD.get_cell(3, 3), Ok(Cell::Filled(9)));
        assert_eq!(
            BOARD.get_cell(4, 4),
            Err(BoardError::RangeError {
                min: 1,
                max: 3,
                value: 4,
            })
        );
    }
}
