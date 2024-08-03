use std::fmt;

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

// Stack allocated 2D array of Cells
// Neighbour counting algorithm is implemented here
#[derive(Debug)]
pub struct CellArray<const H: usize, const W: usize>([[Cell; W]; H]);

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

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl<const H: usize, const W: usize> CellArray<H, W> {
    pub fn new() -> CellArray<H, W> {
        CellArray([[Cell::new(); W]; H])
    }

    // Return a reference to the cell at (x, y)
    pub fn cell(&self, x: isize, y: isize) -> &Cell {
        let wrapped_x = ((x % W as isize + W as isize) % W as isize) as usize;
        let wrapped_y = ((y % H as isize + H as isize) % H as isize) as usize;
        &self.0[wrapped_y][wrapped_x]
    }

    // Return a mutable reference to the cell at (x, y)
    pub fn mut_cell(&mut self, x: isize, y: isize) -> &mut Cell {
        let wrapped_x = ((x % W as isize + W as isize) % W as isize) as usize;
        let wrapped_y = ((y % H as isize + H as isize) % H as isize) as usize;
        &mut self.0[wrapped_y][wrapped_x]
    }

    pub fn reset(&mut self) {
        for i in 0..H {
            for j in 0..W {
                self.0[i][j].clear();
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.0.iter().flat_map(|row| row.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.0.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn rows(&self) -> usize {
        H
    }

    pub fn cols(&self) -> usize {
        W
    }
}

impl<const H: usize, const W: usize> fmt::Display for CellArray<H, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..H {
            for j in 0..W {
                write!(f, "{} ", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    const ARRAY_H: usize = 2;
    const ARRAY_W: usize = 5;

    use super::*;

    fn setup() -> CellArray<ARRAY_H, ARRAY_W> {
        CellArray::<ARRAY_H, ARRAY_W>::new()
    }

    #[test]
    fn test_wrapping_top_left_to_top_right() {
        let mut cell_array = setup();

        // Should wrap to (ARRAY_W - 1, 0)
        let x = -1;
        let cell = cell_array.mut_cell(x, 0);
        cell.spawn();

        let destination = cell_array.cell(4, 0);
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_wrapping_top_right_to_top_left() {
        let mut cell_array = setup();

        // Should wrap to (0, 0)
        let x = ARRAY_W as isize;
        let cell = cell_array.mut_cell(x, 0);
        cell.spawn();

        let destination = cell_array.cell(0, 0);
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_wrapping_bottom_left_to_bottom_right() {
        let mut cell_array = setup();

        // Should wrap to (ARRAY_W - 1, ARRAY_H - 1) which is the bottom-right cell
        let x = -1; // One left of the bottom-left cell
        let y = (ARRAY_H - 1) as isize; // Bottom row
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell((ARRAY_W - 1) as isize, (ARRAY_H - 1) as isize); // Bottom-right cell
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_wrapping_bottom_right_to_bottom_left() {
        let mut cell_array = setup();

        // Should wrap to (0, ARRAY_H - 1) which is the bottom-left cell
        let x = ARRAY_W as isize; // One right of the bottom-right cell
        let y = (ARRAY_H - 1) as isize; // Bottom row
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(0, (ARRAY_H - 1) as isize); // Bottom-left cell
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_top_right_corner_to_bottom_left_corner() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        // Should wrap to (ARRAY_W - 1, ARRAY_H - 1) which is the bottom-right cell
        let x = -1; // One left of the top-left cell
        let y = -1; // One up from the top-left cell
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell((ARRAY_W - 1) as isize, (ARRAY_H - 1) as isize); // Bottom-right cell
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_bottom_left_corner_to_top_right_corner() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        // Should wrap to (0, 0) which is the top-left cell
        let x = ARRAY_W as isize; // One right of the bottom-right cell
        let y = ARRAY_H as isize; // One down from the bottom-right cell
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(0, 0); // Top-left cell
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_spawn() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00000001");
        assert_eq!(*cell == 0b00000001, true);
    }

    #[test]
    fn test_kill() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        cell.kill();
        assert_eq!(cell.is_alive(), false);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(*cell == 0b00000000, true);
    }

    #[test]
    fn test_set_neighbors_0() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(0);
        assert_eq!(cell.neighbour_cnt(), 0);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(*cell == 0b00000000, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00000001");
    }

    #[test]
    fn test_set_neighbors_1() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(1);
        assert_eq!(cell.neighbour_cnt(), 1);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(*cell == 0b00000010, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00000011");
    }

    #[test]
    fn test_set_neighbors_2() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(2);
        assert_eq!(cell.neighbour_cnt(), 2);
        assert_eq!(cell.to_string(), "00000100");
        assert_eq!(*cell == 0b00000100, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00000101");
    }

    #[test]
    fn test_set_neighbors_3() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(3);
        assert_eq!(cell.neighbour_cnt(), 3);
        assert_eq!(cell.to_string(), "00000110");
        assert_eq!(*cell == 0b00000110, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00000111");
    }

    #[test]
    fn test_set_neighbors_4() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(4);
        assert_eq!(cell.neighbour_cnt(), 4);
        assert_eq!(cell.to_string(), "00001000");
        assert_eq!(*cell == 0b00001000, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00001001");

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00001001");
    }

    #[test]
    fn test_set_neighbors_5() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(5);
        assert_eq!(cell.neighbour_cnt(), 5);
        assert_eq!(cell.to_string(), "00001010");
        assert_eq!(*cell == 0b00001010, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00001011");
    }

    #[test]
    fn test_set_neighbors_6() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(6);
        assert_eq!(cell.neighbour_cnt(), 6);
        assert_eq!(cell.to_string(), "00001100");
        assert_eq!(*cell == 0b00001100, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00001101");
    }

    #[test]
    fn test_set_neighbors_7() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(7);
        assert_eq!(cell.neighbour_cnt(), 7);
        assert_eq!(cell.to_string(), "00001110");
        assert_eq!(*cell == 0b00001110, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00001111");
    }

    #[test]
    fn test_set_neighbors_8() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(8);
        assert_eq!(cell.neighbour_cnt(), 8);
        assert_eq!(cell.to_string(), "00010000");
        assert_eq!(*cell == 0b00010000, true);

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00010001");
    }

    #[test]
    fn test_reset() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        cell.set_neighbors(8);
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.neighbour_cnt(), 8);
        assert_eq!(cell.to_string(), "00010001");
        cell_array.reset();
        let cell = cell_array.cell(0, 0);
        assert_eq!(cell.neighbour_cnt(), 0);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(*cell == 0b00000000, true);
    }
}
