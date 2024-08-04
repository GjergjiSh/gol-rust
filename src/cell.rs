use std::{fmt, ptr};

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

    //TODO: Weird behavior
    pub fn increment_neighbour_count(&mut self) {
        let count = (self.0 >> 1) & 0b1111;
        assert!(count <= 8, "Neighbor count must be between 0 and 8");
        // if count == 8 {
        //     return;
        // }
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

//TODO: Copy/Clone vs memcopy
// Stack allocated 2D array of Cells
// Neighbour counting algorithm is implemented here
#[derive(Debug, Copy, Clone)]
pub struct CellArray<const H: usize, const W: usize>([[Cell; W]; H]);

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

    pub fn width(&self) -> usize {
        H
    }

    pub fn height(&self) -> usize {
        W
    }

    pub fn spawn_cell(&mut self, x: isize, y: isize) {
        let neighbour_coordinates = self.neighbour_coordinates(x, y);

        // let nghbr_cnt = self.calc_neighbours(x, y);
        let cell = self.mut_cell(x, y);
        cell.spawn();
        // cell.set_neighbors(nghbr_cnt);

        for (x, y) in neighbour_coordinates.iter() {
            let cell = self.mut_cell(*x, *y);
            cell.increment_neighbour_count();
        }
    }

    pub fn kill_cell(&mut self, x: isize, y: isize) {
        let neighbour_coordinates = self.neighbour_coordinates(x, y);

        // let nghbr_cnt = self.calc_neighbours(x, y);
        let cell = self.mut_cell(x, y);
        cell.kill();
        // cell.set_neighbors(nghbr_cnt);

        for (x, y) in neighbour_coordinates.iter() {
            let cell = self.mut_cell(*x, *y);
            cell.decrement_neighbour_count();
        }
    }

    pub fn neighbour_coordinates(&self, x: isize, y: isize) -> [(isize, isize); 8] {
        [
            (x.wrapping_sub(1), y.wrapping_sub(1)), // top_left
            (x, y.wrapping_sub(1)),                 // top
            (x.wrapping_add(1), y.wrapping_sub(1)), // top_right
            (x.wrapping_sub(1), y),                 // left
            (x.wrapping_add(1), y),                 // right
            (x.wrapping_sub(1), y.wrapping_add(1)), // bottom_left
            (x, y.wrapping_add(1)),                 // bottom
            (x.wrapping_add(1), y.wrapping_add(1)), // bottom_right
        ]
    }

    //TODO: Remove?
    fn calc_neighbours(&self, x: isize, y: isize) -> u8 {
        let mut neighbour_count = 0;

        let neighbors = [
            self.cell(x.wrapping_sub(1), y.wrapping_sub(1)), // top_left
            self.cell(x, y.wrapping_sub(1)),                 // top
            self.cell(x.wrapping_add(1), y.wrapping_sub(1)), // top_right
            self.cell(x.wrapping_sub(1), y),                 // left
            self.cell(x.wrapping_add(1), y),                 // right
            self.cell(x.wrapping_sub(1), y.wrapping_add(1)), // bottom_left
            self.cell(x, y.wrapping_add(1)),                 // bottom
            self.cell(x.wrapping_add(1), y.wrapping_add(1)), // bottom_right
        ];

        for neighbor in neighbors.iter() {
            if neighbor.is_alive() {
                neighbour_count += 1;
            }
        }

        neighbour_count
    }

    //TODO: This can be parallelized?
    pub fn memcopy(&mut self, other: &mut Self) {
        unsafe {
            ptr::copy_nonoverlapping(self.0.as_ptr(), other.0.as_mut_ptr(), H * W);
        }
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
    const ARRAY_H: usize = 5;
    const ARRAY_W: usize = 5;

    use std::cell;

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
    fn test_top_right_to_bottom_left_corner() {
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
    fn test_bottom_left_to_top_right_corner() {
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
    fn test_top_left_to_bottom_left_corner() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = 0;
        let y = -1;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(0, (ARRAY_H - 1) as isize); // Bottom-left cell
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_top_right_to_bottom_right() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = ARRAY_W as isize - 1;
        let y = -1;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(ARRAY_W as isize - 1, ARRAY_H as isize - 1);
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_bottom_left_to_top_left() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = 0;
        let y = ARRAY_H as isize;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(0, 0);
        assert_eq!(destination.is_alive(), true);
    }

    #[test]
    fn test_bottom_right_to_top_right() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = ARRAY_W as isize - 1;
        let y = ARRAY_H as isize;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(ARRAY_W as isize - 1, 0);
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
    fn test_increment_neighbours() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.increment_neighbour_count();
        assert_eq!(cell.neighbour_cnt(), 1);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(*cell == 0b00000010, true);
    }

    #[test]
    fn test_decrement_neighbours() {
        let mut cell_array = setup();
        let cell = cell_array.mut_cell(0, 0);
        cell.set_neighbors(1);
        assert_eq!(cell.neighbour_cnt(), 1);
        assert_eq!(cell.to_string(), "00000010");
        assert_eq!(*cell == 0b00000010, true);
        cell.decrement_neighbour_count();
        assert_eq!(cell.neighbour_cnt(), 0);
        assert_eq!(cell.to_string(), "00000000");
        assert_eq!(*cell == 0b00000000, true);
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

    #[test]
    fn test_glider() {
        let mut cell_array = setup();

        let x = 0;
        let y = 0;
        let pattern_coords = [
            (x + 2, y),
            (x + 2, y + 1),
            (x + 2, y + 2),
            (x + 1, y + 2),
            (x, y + 1),
        ];

        for &(x, y) in &pattern_coords {
            cell_array.spawn_cell(x, y);
        }

        //First column
        let c1_neighbours = cell_array.calc_neighbours(0, 0);
        let c1 = cell_array.mut_cell(0, 0);

        assert_eq!(c1.is_alive(), false);
        assert_eq!(c1_neighbours, 1);
        assert_eq!(c1.to_string(), "00000010");
        assert_eq!(*c1 == 0b00000010, true);

        let c2_neighbours = cell_array.calc_neighbours(0, 1);
        let c2 = cell_array.mut_cell(0, 1);

        assert_eq!(c2.is_alive(), true);
        assert_eq!(c2_neighbours, 1);
        assert_eq!(c2.to_string(), "00000011");
        assert_eq!(*c2 == 0b00000011, true);

        let c3 = cell_array.cell(0, 2);
        let c3_neighbours = cell_array.calc_neighbours(0, 2);
        assert_eq!(c3.is_alive(), false);
        assert_eq!(c3_neighbours, 2);

        let c4 = cell_array.cell(0, 3);
        let c4_neighbours = cell_array.calc_neighbours(0, 3);
        assert_eq!(c4.is_alive(), false);
        assert_eq!(c4_neighbours, 1);

        let c5 = cell_array.cell(0, 4);
        let c5_neighbours = cell_array.calc_neighbours(0, 4);
        assert_eq!(c5.is_alive(), false);
        assert_eq!(c5_neighbours, 0);

        //Second column
        let c6 = cell_array.cell(1, 0);
        let c6_neighbours = cell_array.calc_neighbours(1, 0);
        assert_eq!(c6.is_alive(), false);
        assert_eq!(c6_neighbours, 3);

        let c7 = cell_array.cell(1, 1);
        let c7_neighbours = cell_array.calc_neighbours(1, 1);
        assert_eq!(c7.is_alive(), false);
        assert_eq!(c7_neighbours, 5);

        let c8 = cell_array.cell(1, 2);
        let c8_neighbours = cell_array.calc_neighbours(1, 2);
        assert_eq!(c8.is_alive(), true);
        assert_eq!(c8_neighbours, 3);

        let c9 = cell_array.cell(1, 3);
        let c9_neighbours = cell_array.calc_neighbours(1, 3);
        assert_eq!(c9.is_alive(), false);
        assert_eq!(c9_neighbours, 2);

        let c10 = cell_array.cell(1, 4);
        let c10_neighbours = cell_array.calc_neighbours(1, 4);
        assert_eq!(c10.is_alive(), false);
        assert_eq!(c10_neighbours, 1);

        //Third column
        let c11 = cell_array.cell(2, 0);
        let c11_neighbours = cell_array.calc_neighbours(2, 0);
        assert_eq!(c11.is_alive(), true);
        assert_eq!(c11_neighbours, 1);

        let c12 = cell_array.cell(2, 1);
        let c12_neighbours = cell_array.calc_neighbours(2, 1);
        assert_eq!(c12.is_alive(), true);
        assert_eq!(c12_neighbours, 3);

        let c13 = cell_array.cell(2, 2);
        let c13_neighbours = cell_array.calc_neighbours(2, 2);
        assert_eq!(c13.is_alive(), true);
        assert_eq!(c13_neighbours, 2);

        let c14 = cell_array.cell(2, 3);
        let c14_neighbours = cell_array.calc_neighbours(2, 3);
        assert_eq!(c14.is_alive(), false);
        assert_eq!(c14_neighbours, 2);

        let c15 = cell_array.cell(2, 4);
        let c15_neighbours = cell_array.calc_neighbours(2, 4);
        assert_eq!(c15.is_alive(), false);
        assert_eq!(c15_neighbours, 1);

        //Fourth column
        let c16 = cell_array.cell(3, 0);
        let c16_neighbours = cell_array.calc_neighbours(3, 0);
        assert_eq!(c16.is_alive(), false);
        assert_eq!(c16_neighbours, 2);

        let c17 = cell_array.cell(3, 1);
        let c17_neighbours = cell_array.calc_neighbours(3, 1);
        assert_eq!(c17.is_alive(), false);
        assert_eq!(c17_neighbours, 3);

        let c18 = cell_array.cell(3, 2);
        let c18_neighbours = cell_array.calc_neighbours(3, 2);
        assert_eq!(c18.is_alive(), false);
        assert_eq!(c18_neighbours, 2);

        let c19 = cell_array.cell(3, 3);
        let c19_neighbours = cell_array.calc_neighbours(3, 3);
        assert_eq!(c19.is_alive(), false);
        assert_eq!(c19_neighbours, 1);

        let c20 = cell_array.cell(3, 4);

        let c20_neighbours = cell_array.calc_neighbours(3, 4);
        assert_eq!(c20.is_alive(), false);
        assert_eq!(c20_neighbours, 1);

        //Fifth column
        let c21 = cell_array.cell(4, 0);
        let c21_neighbours = cell_array.calc_neighbours(4, 0);
        assert_eq!(c21.is_alive(), false);
        assert_eq!(c21_neighbours, 1);

        let c22 = cell_array.cell(4, 1);
        let c22_neighbours = cell_array.calc_neighbours(4, 1);
        assert_eq!(c22.is_alive(), false);
        assert_eq!(c22_neighbours, 1);

        let c23 = cell_array.cell(4, 2);
        let c23_neighbours = cell_array.calc_neighbours(4, 2);
        assert_eq!(c23.is_alive(), false);
        assert_eq!(c23_neighbours, 1);

        let c24 = cell_array.cell(4, 3);
        let c24_neighbours = cell_array.calc_neighbours(4, 3);
        assert_eq!(c24.is_alive(), false);
        assert_eq!(c24_neighbours, 0);

        let c25 = cell_array.cell(4, 4);
        let c25_neighbours = cell_array.calc_neighbours(4, 4);
        assert_eq!(c25.is_alive(), false);
        assert_eq!(c25_neighbours, 0);
    }

    /* #[test]
    fn test_memcopy() {
        let mut cell_array = setup();
        let mut other = setup();

        let cell = cell_array.mut_cell(0, 0);
        cell.spawn();
        cell.set_neighbors(8);

        cell_array.memcopy(&mut other);

        let cell = other.cell(0, 0);
        assert_eq!(cell.neighbour_cnt(), 8);
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00010001");
        assert_eq!(*cell == 0b00010001, true);
    } */
}
