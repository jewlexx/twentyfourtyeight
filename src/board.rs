use std::ops::RangeInclusive;

use rand::Rng;

mod cell;
pub mod errors;

use errors::*;

pub use cell::*;

const ROCOLMIN: usize = 1;
const ROCOLMAX: usize = 4;
const ROCOLRANGE: RangeInclusive<usize> = 1..=4;

#[derive(Debug, derive_more::Display, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    /// 13->9->5->1 & 14->10->6->2 & 15->11->7->3 & 16->12->8->4
    Up,
    /// 1->5->9->13 & 2->6->10->14 & 3->7->11->15 & 4->8->12->16
    Down,
    /// 1->2->3->4 & 5->6->7->8 & 9->10->11->12 & 13->14->15->16
    Left,
    /// 4->3->2->1 & 8->7->6->5 & 12->11->10->9 & 16->15->14->13
    Right,
}

#[derive(Debug, derive_more::Display, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    /// Left or Right
    X,
    /// Up or Down
    Y,
}

impl From<Direction> for Axis {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up | Direction::Down => Axis::Y,
            Direction::Left | Direction::Right => Axis::X,
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = BoardError;

    fn try_from(str: &str) -> Result<Self> {
        match str {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            _ => Err(BoardError::InvalidDirection {
                direction: str.to_string(),
            }),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [Cell; 16],
}

impl Board {
    pub const fn new(cells: [Cell; 16]) -> Self {
        Board { cells }
    }

    pub const fn empty() -> Self {
        Board::new([Cell::Empty; 16])
    }

    fn move_y(&mut self, direction: &Direction) -> Result<()> {
        let columns = self.get_columns()?;

        for (i, column) in columns.iter().enumerate() {
            match direction {
                Direction::Up => {
                    if i != 0 {
                        let previous = columns[i - 1];

                        for (j, cell) in column.iter().enumerate() {
                            if cell.is_empty() {
                                continue;
                            }

                            let mut k = j;

                            while k > 0 {
                                let previous_cell = previous[k - 1];

                                if previous_cell.is_empty() {
                                    self.cells[(k - 1) * 4 + i] = *cell;
                                    self.cells[k * 4 + i] = Cell::Empty;
                                } else if previous_cell == *cell {
                                    self.cells[(k - 1) * 4 + i] = *cell * 2;
                                    self.cells[k * 4 + i] = Cell::Empty;
                                }

                                k -= 1;
                            }
                        }
                    }
                }
                Direction::Down => {}
                _ => return Err(BoardError::InvalidMoveDirection(*direction, Axis::Y)),
            };
        }

        Ok(())
    }

    pub fn update(&mut self, direction: impl Into<Direction>) -> Result<()> {
        let direction: Direction = direction.try_into()?;
        let axis: Axis = direction.into();

        Ok(())
    }

    pub fn gen_new() -> Result<Self> {
        let mut board = Board::empty();

        let mut rng = rand::thread_rng();

        let index = {
            let row = rng.gen_range(ROCOLRANGE);
            let column = rng.gen_range(ROCOLRANGE);

            (row, column)
        };

        let mut second_index = index;

        while second_index == index {
            second_index = {
                let row = rng.gen_range(ROCOLRANGE);
                let column = rng.gen_range(ROCOLRANGE);

                (row, column)
            };
        }

        board.gen_cell(index.0, index.1)?;
        board.gen_cell(second_index.0, second_index.1)?;

        Ok(board)
    }

    pub fn get_filled(&self) -> Vec<Cell> {
        self.cells
            .iter()
            .filter(|c| c.is_filled())
            .copied()
            .collect()
    }

    pub fn get_filled_count(&self) -> usize {
        self.get_filled().len()
    }

    pub const fn get_cells(&self) -> &[Cell; 16] {
        &self.cells
    }

    pub fn get_rows(&self) -> Result<[[Cell; 4]; 4]> {
        let mut rows = [[Cell::Empty; 4]; 4];

        for (i, row) in rows.iter_mut().enumerate() {
            *row = self.get_row(i + 1)?;
        }

        Ok(rows)
    }

    pub fn get_row(&self, row: usize) -> Result<[Cell; 4]> {
        if !(ROCOLRANGE).contains(&row) {
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: row,
            })
        } else {
            // Forces the array to be sized
            let row = &self.cells[(row - 1) * 4..row * 4];
            Ok([row[0], row[1], row[2], row[3]])
        }
    }

    pub fn get_columns(&self) -> Result<[[Cell; 4]; 4]> {
        let mut columns = [[Cell::Empty; 4]; 4];

        for (i, row) in columns.iter_mut().enumerate() {
            *row = self.get_column(i + 1)?;
        }

        Ok(columns)
    }

    pub fn get_column(&self, column: usize) -> Result<[Cell; 4]> {
        if !(ROCOLRANGE).contains(&column) {
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: column,
            })
        } else {
            Ok([
                self.cells[column - 1],
                self.cells[column + 3],
                self.cells[column + 7],
                self.cells[column + 11],
            ])
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Result<Cell> {
        if !(ROCOLRANGE).contains(&row) {
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: row,
            })
        } else if !(ROCOLRANGE).contains(&column) {
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: column,
            })
        } else {
            Ok(self.cells[(row - 1) * 3 + column - 1])
        }
    }

    pub fn gen_cell(&mut self, row: usize, column: usize) -> Result<()> {
        if !(ROCOLRANGE).contains(&row) {
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: row,
            })
        } else if !(ROCOLRANGE).contains(&column) {
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: column,
            })
        } else {
            let mut cell = self.get_cell(row, column)?;
            cell.gen_filled_cell();

            self.set_cell(row, column, cell)?;

            Ok(())
        }
    }

    pub fn set_cell(&mut self, row: usize, column: usize, value: Cell) -> Result<()> {
        if !(ROCOLRANGE).contains(&row) {
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: row,
            })
        } else if !(ROCOLRANGE).contains(&column) {
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: column,
            })
        } else {
            self.cells[(row - 1) * 3 + column - 1] = value;

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust incorrectly warns about dead code in the following two declarations
    #[allow(dead_code)]
    const CELLS: [Cell; 16] = [
        Cell::Filled(1),
        Cell::Filled(2),
        Cell::Filled(3),
        Cell::Filled(4),
        Cell::Filled(5),
        Cell::Filled(6),
        Cell::Filled(7),
        Cell::Filled(8),
        Cell::Filled(9),
        Cell::Filled(10),
        Cell::Filled(11),
        Cell::Filled(12),
        Cell::Filled(13),
        Cell::Filled(14),
        Cell::Filled(15),
        Cell::Filled(16),
    ];

    #[allow(dead_code)]
    static BOARD: Board = Board { cells: CELLS };

    #[test]
    fn test_gen_board() {
        let board = Board::gen_new().unwrap();

        assert_eq!(board.get_filled_count(), 2);

        let filled = board.get_filled();

        assert!(filled[0].get_value() == Some(2) || filled[0].get_value() == Some(4));

        assert!(filled[1].get_value() == Some(2) || filled[1].get_value() == Some(4));
    }

    #[test]
    fn test_new() {
        let board = Board::new(CELLS);

        assert_eq!(board.get_cells(), &CELLS);
    }

    #[test]
    fn test_empty() {
        let board = Board::empty();

        assert_eq!(board.get_cells(), &[Cell::Empty; 16]);
    }

    #[test]
    fn test_get_row() {
        let rows = [
            [
                Cell::Filled(1),
                Cell::Filled(2),
                Cell::Filled(3),
                Cell::Filled(4),
            ],
            [
                Cell::Filled(5),
                Cell::Filled(6),
                Cell::Filled(7),
                Cell::Filled(8),
            ],
            [
                Cell::Filled(9),
                Cell::Filled(10),
                Cell::Filled(11),
                Cell::Filled(12),
            ],
            [
                Cell::Filled(13),
                Cell::Filled(14),
                Cell::Filled(15),
                Cell::Filled(16),
            ],
        ];

        assert_eq!(BOARD.get_row(1), Ok(rows[0]));
        assert_eq!(BOARD.get_row(2), Ok(rows[1]));
        assert_eq!(BOARD.get_row(3), Ok(rows[2]));
        assert_eq!(BOARD.get_row(4), Ok(rows[3]));
        assert_eq!(BOARD.get_rows(), Ok(rows));

        assert_eq!(
            BOARD.get_row(5),
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: 5,
            })
        );
    }

    #[test]
    fn test_get_column() {
        let columns = [
            [
                Cell::Filled(1),
                Cell::Filled(5),
                Cell::Filled(9),
                Cell::Filled(13),
            ],
            [
                Cell::Filled(2),
                Cell::Filled(6),
                Cell::Filled(10),
                Cell::Filled(14),
            ],
            [
                Cell::Filled(3),
                Cell::Filled(7),
                Cell::Filled(11),
                Cell::Filled(15),
            ],
            [
                Cell::Filled(4),
                Cell::Filled(8),
                Cell::Filled(12),
                Cell::Filled(16),
            ],
        ];

        assert_eq!(BOARD.get_column(1), Ok(columns[0]));
        assert_eq!(BOARD.get_column(2), Ok(columns[1]));
        assert_eq!(BOARD.get_column(3), Ok(columns[2]));
        assert_eq!(BOARD.get_column(4), Ok(columns[3]));
        assert_eq!(BOARD.get_columns(), Ok(columns));

        assert_eq!(
            BOARD.get_column(5),
            Err(BoardError::RangeError {
                min: ROCOLMIN,
                max: ROCOLMAX,
                value: 5,
            })
        );
    }

    #[test]
    fn test_get_cell() {
        assert_eq!(BOARD.get_cell(1, 1), Ok(Cell::Filled(1)));
        assert_eq!(BOARD.get_cell(2, 2), Ok(Cell::Filled(5)));
        assert_eq!(BOARD.get_cell(3, 3), Ok(Cell::Filled(9)));
        assert_eq!(BOARD.get_cell(4, 4), Ok(Cell::Filled(13)));
        assert_eq!(BOARD.get_cells(), &CELLS);

        assert_eq!(
            BOARD.get_cell(5, 5),
            Err(BoardError::RangeError {
                min: 1,
                max: 4,
                value: 5,
            })
        );
    }
}
