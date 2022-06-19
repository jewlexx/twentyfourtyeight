use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Filled(u128),
    Empty,
}

impl Cell {
    pub fn new() -> Self {
        let mut cell = Self::empty();

        cell.gen_filled_cell();

        cell
    }

    pub fn empty() -> Self {
        Cell::Empty
    }

    pub fn gen_filled_cell(&mut self) {
        let mut rng = rand::thread_rng();

        let value = if rng.gen_bool(0.5) { 2 } else { 4 };

        self.set_value(value);
    }

    pub fn is_filled(&self) -> bool {
        match self {
            Cell::Filled(_) => true,
            Cell::Empty => false,
        }
    }

    pub fn get_value(&self) -> Option<u128> {
        match self {
            Cell::Filled(value) => Some(*value),
            Cell::Empty => None,
        }
    }

    pub fn set_value(&mut self, value: u128) {
        match self {
            Cell::Filled(_) => *self = Cell::Filled(value),
            Cell::Empty => *self = Cell::Filled(value),
        }
    }
}
