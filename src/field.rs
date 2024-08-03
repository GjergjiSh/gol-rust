use std::fmt;

// Wrapper around a u8.
// Represents the state of a cell.
// Offers simple API for manipulating the state via bitwise operations.
// The first bit is the state of the cell (0 = dead, 1 = alive)
// The next 4 bits are the number of neighbors
// The last 3 bits are unused
//  [x, x, x, |0, 0, 0, 0, |1] -> Alive cell with 0 neighbors
//  [x, x, x, |1, 0, 0, 0, |0] -> Dead cell with 8 neighbors
#[derive(Debug, Copy, Clone)]
pub struct Cell(u8);

impl Cell {
    fn new() -> Cell {
        Cell(0)
    }

    pub fn spawn(&mut self) {
        self.0 |= 1;
    }

    pub fn kill(&mut self) {
        self.0 &= 0;
    }

    pub fn set_neighbors(&mut self, count: u8) {
        assert!(count <= 8, "Neighbor count must be between 0 and 8");
        // Clear the 4 bits following the first bit
        self.0 &= 0b1110_0001;
        // Set the 4 bits following the first bit to the count
        self.0 |= (count << 1) & 0b0001_1110;
    }

    pub fn neighbour_cnt(&self) -> u8 {
        (self.0 >> 1) & 0b0000_1111
    }

    pub fn is_alive(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

impl PartialEq<u8> for Cell {
    fn eq(&self, other: &u8) -> bool {
        &self.0 == other
    }
}

// Stack allocated 2D array of Cells
// Neighbour counting algorithm is implemented here
#[derive(Debug)]
pub struct Field<const H: usize, const W: usize> {
    cells: [[Cell; W]; H],
}

impl<const H: usize, const W: usize> Field<H, W> {
    pub fn new() -> Field<H, W> {
        Field {
            cells: [[Cell::new(); W]; H],
        }
    }

    // Return a reference to the cell at (x, y)
    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        assert_eq!(x < W, true);
        assert_eq!(y < H, true);
        &self.cells[y][x]
    }

    // Return a mutable reference to the cell at (x, y)
    pub fn mut_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        assert_eq!(x < W, true);
        assert_eq!(y < H, true);
        &mut self.cells[y][x]
    }

    pub fn reset(&mut self) {
        for i in 0..H {
            for j in 0..W {
                self.cells[i][j].clear();
            }
        }
    }
}

// Debug/Visualization
impl<const H: usize, const W: usize> fmt::Display for Field<H, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..H {
            for j in 0..W {
                write!(f, "{} ", self.cells[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Field<2, 5> {
        Field::<2, 5>::new()
    }

    #[test]
    fn test_spawn() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00000001");
        assert_eq!(*cell == 0b00000001, true);
    }

    #[test]
    fn test_kill() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.spawn();
        cell.kill();
        assert_eq!(cell.is_alive(), false);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(*cell == 0b00000000, true);
    }

    #[test]
    fn test_set_neighbors_0() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(0);
        assert_eq!(cell.neighbour_cnt(), 0);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(*cell == 0b00000000, true);
    }

    #[test]
    fn test_set_neighbors_1() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(1);
        assert_eq!(cell.neighbour_cnt(), 1);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(*cell == 0b00000010, true);
    }

    #[test]
    fn test_set_neighbors_2() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(2);
        assert_eq!(cell.neighbour_cnt(), 2);
        assert_eq!(cell.to_string(), "00000100");
        assert_eq!(*cell == 0b00000100, true);
    }

    #[test]
    fn test_set_neighbors_3() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(3);
        assert_eq!(cell.neighbour_cnt(), 3);
        assert_eq!(cell.to_string(), "00000110");
        assert_eq!(*cell == 0b00000110, true);
    }

    #[test]
    fn test_set_neighbors_4() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(4);
        assert_eq!(cell.neighbour_cnt(), 4);
        assert_eq!(cell.to_string(), "00001000");
        assert_eq!(*cell == 0b00001000, true);
    }

    #[test]
    fn test_set_neighbors_5() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(5);
        assert_eq!(cell.neighbour_cnt(), 5);
        assert_eq!(cell.to_string(), "00001010");
        assert_eq!(*cell == 0b00001010, true);
    }

    #[test]
    fn test_set_neighbors_6() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(6);
        assert_eq!(cell.neighbour_cnt(), 6);
        assert_eq!(cell.to_string(), "00001100");
        assert_eq!(*cell == 0b00001100, true);
    }

    #[test]
    fn test_set_neighbors_7() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(7);
        assert_eq!(cell.neighbour_cnt(), 7);
        assert_eq!(cell.to_string(), "00001110");
        assert_eq!(*cell == 0b00001110, true);
    }

    #[test]
    fn test_set_neighbors_8() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.set_neighbors(8);
        assert_eq!(cell.neighbour_cnt(), 8);
        assert_eq!(cell.to_string(), "00010000");
        assert_eq!(*cell == 0b00010000, true);
    }

    #[test]
    fn test_reset() {
        let mut field = setup();
        let cell = field.mut_cell(0, 0);
        cell.spawn();
        cell.set_neighbors(8);
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.neighbour_cnt(), 8);
        assert_eq!(cell.to_string(), "00010001");
        field.reset();
        let cell = field.cell(0, 0);
        assert_eq!(cell.neighbour_cnt(), 0);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(*cell == 0b00000000, true);
    }
}