use std::{fmt, ptr};

use crate::gol::cell::Cell;

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
    pub fn calc_neighbours(&self, x: isize, y: isize) -> u8 {
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
mod test_cell_array {
    use super::CellArray;

    const ARRAY_H: usize = 5;
    const ARRAY_W: usize = 5;

    fn setup() -> CellArray<ARRAY_H, ARRAY_W> {
        CellArray::<ARRAY_H, ARRAY_W>::new()
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

