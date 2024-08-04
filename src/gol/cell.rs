use std::{fmt, ptr};

use crate::gol::cell_array::CellArray;

// Wrapper around a u8.
// Represents the state of a cell.
// Offers simple API for manipulating the state via bitwise operations.
// The first bit is the state of the cell (0 = dead, 1 = alive)
// The next 4 bits are the number of neighbors in binary
// The last 3 bits are unused
//  [x, x, x, |0, 0, 0, 0, |1] -> Alive cell with 0 neighbors
//  [x, x, x, |1, 0, 0, 0, |0] -> Dead cell with 8 neighbors
#[derive(Debug, Copy, Clone)]
pub struct Cell(u8);

impl Cell {
    pub fn new() -> Cell {
        Cell(0)
    }

    pub fn spawn(&mut self) {
        self.0 |= 1;
    }

    pub fn kill(&mut self) {
        self.0 &= !1;
    }

    pub fn neighbour_cnt(&self) -> u8 {
        (self.0 >> 1) & 0b0000_1111
    }

    pub fn alive(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn increment_neighbour_count(&mut self) {
        let count = (self.0 >> 1) & 0b1111;
        assert!(count + 1 <= 8, "Neighbor count must be between 0 and 8");
        self.0 = (self.0 & 0b0000_0001) | ((count + 1) << 1);
    }

    pub fn decrement_neighbour_count(&mut self) {
        let count = (self.0 >> 1) & 0b1111;
        if count == 0 {
            return;
        }
        // assert!(count >= 0, "Neighbor count must be between 0 and 8");
        self.0 = (self.0 & 0b0000_0001) | ((count - 1) << 1);
    }
}

impl PartialEq<u8> for Cell {
    fn eq(&self, other: &u8) -> bool {
        &self.0 == other
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

#[cfg(test)]
mod test_cell {
    use super::*;

    #[test]
    fn test_spawn() {
        let mut cell = Cell::new();
        cell.increment_neighbour_count();
        assert_eq!(cell, 0b00000010);
        cell.spawn();
        assert_eq!(cell.alive(), true);
        assert_eq!(cell.to_string(), "00000011");
        assert_eq!(cell == 0b00000011, true);
    }

    #[test]
    fn test_kill() {
        let mut cell = Cell::new();
        cell.spawn();
        cell.increment_neighbour_count();
        cell.kill();
        assert_eq!(cell.alive(), false);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(cell == 0b00000010, true);
    }

    #[test]
    fn test_increment_neighbours() {
        let mut cell = Cell::new();
        cell.increment_neighbour_count();
        assert_eq!(cell.neighbour_cnt(), 1);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(cell == 0b00000010, true);

        cell.increment_neighbour_count();
        assert_eq!(cell.neighbour_cnt(), 2);
        assert_eq!(cell.to_string(), "00000100");
        assert_eq!(cell == 0b00000100, true);

        cell.increment_neighbour_count();
        assert_eq!(cell.neighbour_cnt(), 3);
        assert_eq!(cell.to_string(), "00000110");
        assert_eq!(cell == 0b00000110, true);
    }

    #[test]
    fn test_decrement_neighbours() {
        let mut cell = Cell::new();
        cell.increment_neighbour_count();
        assert_eq!(cell.neighbour_cnt(), 1);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(cell == 0b00000010, true);
        cell.decrement_neighbour_count();
        assert_eq!(cell.neighbour_cnt(), 0);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(cell == 0b00000000, true);
    }
}
