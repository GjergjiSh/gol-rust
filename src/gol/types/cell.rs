use std::fmt;

// Wrapper around a u8.
// Represents the state of a cell.
// Offers simple API for manipulating the state via bitwise operations.
// The first bit is the state of the cell (0 = dead, 1 = alive)
// The next 4 bits are the number of neighbors in binary
// The last 3 bits are unused
//  [x, x, x, |0, 0, 0, 0, |1] -> Alive cell with 0 neighbors
//  [x, x, x, |1, 0, 0, 0, |0] -> Dead cell with 8 neighbors
#[derive(Debug, Clone)]
pub struct Cell(Box<u8>);

impl Cell {
    pub fn new() -> Cell {
        Cell(Box::new(0))
    }

    // Bitwise operation to set the first bit to 1
    pub fn spawn(&mut self) {
        *self.0 |= 1;
    }

    // Bitwise operation to set the first bit to 0
    pub fn kill(&mut self) {
        *self.0 &= !1;
    }

    // Bitwise operation to check if the first bit is 1
    pub fn alive(&self) -> bool {
        *self.0 & 1 == 1
    }

    // Bitwise operation to get the number of neighbors
    pub fn neighbours(&self) -> u8 {
        (*self.0 >> 1) & 0b0000_1111
    }

    // Bitwise operation to increment the number of neighbors
    pub fn add_neighbour(&mut self) {
        let count = (*self.0 >> 1) & 0b1111;
        assert!(count + 1 <= 8, "Neighbor count must be between 0 and 8");
        *self.0 = (*self.0 & 0b0000_0001) | ((count + 1) << 1);
    }

    // Bitwise operation to decrement the number of neighbors
    pub fn remove_neighbour(&mut self) {
        let count = (*self.0 >> 1) & 0b1111;
        // if count == 0 {
        //     return;
        // }
        // TODO: This part of the code does not behave as intended.
        // assert!(count >= 0, "Neighbor count must be between 0 and 8");
        *self.0 = (*self.0 & 0b0000_0001) | ((count - 1) << 1);
    }
}

impl PartialEq<u8> for Cell {
    fn eq(&self, other: &u8) -> bool {
        *self.0 == *other
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", *self.0)
    }
}

#[cfg(test)]
mod test_cell {
    use super::*;

    #[test]
    fn test_spawn() {
        let mut cell = Cell::new();
        cell.add_neighbour();
        assert_eq!(cell, 0b00000010);
        assert_eq!(cell.alive(), false);
        assert_eq!(cell.neighbours(), 1);
        assert_eq!(cell.to_string(), "00000010");
        cell.spawn();
        assert_eq!(cell.alive(), true);
        assert_eq!(cell.neighbours(), 1);
        assert_eq!(cell.to_string(), "00000011");
        assert_eq!(cell == 0b00000011, true);
    }

    #[test]
    fn test_kill() {
        let mut cell = Cell::new();
        cell.spawn();
        cell.add_neighbour();
        cell.kill();
        assert_eq!(cell.alive(), false);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(cell == 0b00000010, true);
    }

    #[test]
    fn test_increment_neighbours() {
        let mut cell = Cell::new();
        cell.add_neighbour();
        assert_eq!(cell.neighbours(), 1);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(cell == 0b00000010, true);

        cell.add_neighbour();
        assert_eq!(cell.neighbours(), 2);
        assert_eq!(cell.to_string(), "00000100");
        assert_eq!(cell == 0b00000100, true);

        cell.add_neighbour();
        assert_eq!(cell.neighbours(), 3);
        assert_eq!(cell.to_string(), "00000110");
        assert_eq!(cell == 0b00000110, true);
    }

    #[test]
    fn test_decrement_neighbours() {
        let mut cell = Cell::new();
        cell.add_neighbour();
        assert_eq!(cell.neighbours(), 1);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(cell == 0b00000010, true);
        cell.remove_neighbour();
        assert_eq!(cell.neighbours(), 0);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(cell == 0b00000000, true);
    }
}
