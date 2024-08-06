use std::fmt;

use crate::gol::types::Cell;

// Stack allocated 2D array of Cells
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

    pub fn rows(&self) -> usize {
        H
    }

    pub fn cols(&self) -> usize {
        W
    }

    pub fn spawn(&mut self, x: isize, y: isize) {
        let neighbour_coordinates = self.neighbour_coordinates(x, y);

        let cell = self.mut_cell(x, y);
        cell.spawn();

        for (nx, ny) in neighbour_coordinates.iter() {
            let neighbour_cell = self.mut_cell(*nx, *ny);
            neighbour_cell.add_neighbour();
        }
    }

    pub fn kill_cell(&mut self, x: isize, y: isize) {
        let neighbour_coordinates = self.neighbour_coordinates(x, y);

        let cell = self.mut_cell(x, y);
        cell.kill();

        for (nx, ny) in neighbour_coordinates.iter() {
            let neighbour_cell = self.mut_cell(*nx, *ny);
            neighbour_cell.remove_neighbour();
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

    #[allow(dead_code)]
    pub fn print(&self) {
        // Print the top border with column indices
        print!("   "); // Space for row indices
        println!();

        // Print the top border of the grid with column numbers
        print!("  +");
        for x in 0..W {
            print!("-{}-+", x); // Col index
        }
        println!();

        // Print the field with side borders and row indices
        for y in 0..H {
            print!("{:2}|", y); // Row index
            for x in 0..W {
                let cell = self.cell(x as isize, y as isize);
                let symbol = if cell.alive() { '*' } else { ' ' };
                print!(" {} |", symbol);
            }
            println!(); // End of the row with a side border

            // Print the horizontal border between rows without column numbers
            print!("  +");
            for _ in 0..H {
                print!("---+");
            }
            println!();
        }

        println!();
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
    fn test_wrapping_top_left_to_top_right() {
        let mut cell_array = setup();

        // Should wrap to (ARRAY_W - 1, 0)
        let x = -1;
        let cell = cell_array.mut_cell(x, 0);
        cell.spawn();

        let destination = cell_array.cell(4, 0);
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_wrapping_top_right_to_top_left() {
        let mut cell_array = setup();

        // Should wrap to (0, 0)
        let x = ARRAY_W as isize;
        let cell = cell_array.mut_cell(x, 0);
        cell.spawn();

        let destination = cell_array.cell(0, 0);
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_wrapping_bottom_left_to_bottom_right() {
        let mut cell_array = setup();

        let x = -1;
        let y = (ARRAY_H - 1) as isize;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell((ARRAY_W - 1) as isize, (ARRAY_H - 1) as isize); // Bottom-right cell
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_wrapping_bottom_right_to_bottom_left() {
        let mut cell_array = setup();

        let x = ARRAY_W as isize;
        let y = (ARRAY_H - 1) as isize;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(0, (ARRAY_H - 1) as isize); // Bottom-left cell
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_top_right_to_bottom_left_corner() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = -1;
        let y = -1;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell((ARRAY_W - 1) as isize, (ARRAY_H - 1) as isize); // Bottom-right cell
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_bottom_left_to_top_right_corner() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = ARRAY_W as isize;
        let y = ARRAY_H as isize;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(0, 0);
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_top_left_to_bottom_left_corner() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = 0;
        let y = -1;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(0, (ARRAY_H - 1) as isize);
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_top_right_to_bottom_right() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = ARRAY_W as isize - 1;
        let y = -1;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(ARRAY_W as isize - 1, ARRAY_H as isize - 1);
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_bottom_left_to_top_left() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = 0;
        let y = ARRAY_H as isize;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(0, 0);
        assert_eq!(destination.alive(), true);
    }

    #[test]
    fn test_bottom_right_to_top_right() {
        let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();

        let x = ARRAY_W as isize - 1;
        let y = ARRAY_H as isize;
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();

        let destination = cell_array.cell(ARRAY_W as isize - 1, 0);
        assert_eq!(destination.alive(), true);
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
            cell_array.spawn(x, y);
        }

        //First column
        let c1 = cell_array.mut_cell(0, 0);
        let c1_neighbours = c1.neighbours();

        assert_eq!(c1.alive(), false);
        assert_eq!(c1_neighbours, 1);
        assert_eq!(c1.to_string(), "00000010");
        assert_eq!(*c1 == 0b00000010, true);

        let c2 = cell_array.mut_cell(0, 1);
        let c2_neighbours = c2.neighbours();

        assert_eq!(c2.alive(), true);
        assert_eq!(c2_neighbours, 1);
        assert_eq!(c2.to_string(), "00000011");
        assert_eq!(*c2 == 0b00000011, true);

        let c3 = cell_array.cell(0, 2);
        let c3_neighbours = c3.neighbours();
        assert_eq!(c3.alive(), false);
        assert_eq!(c3_neighbours, 2);

        let c4 = cell_array.cell(0, 3);
        let c4_neighbours = c4.neighbours();
        assert_eq!(c4.alive(), false);
        assert_eq!(c4_neighbours, 1);

        let c5 = cell_array.cell(0, 4);
        let c5_neighbours = c5.neighbours();
        assert_eq!(c5.alive(), false);
        assert_eq!(c5_neighbours, 0);

        //Second column
        let c6 = cell_array.cell(1, 0);
        let c6_neighbours = c6.neighbours();
        assert_eq!(c6.alive(), false);
        assert_eq!(c6_neighbours, 3);

        let c7 = cell_array.cell(1, 1);
        let c7_neighbours = c7.neighbours();
        assert_eq!(c7.alive(), false);
        assert_eq!(c7_neighbours, 5);

        let c8 = cell_array.cell(1, 2);
        let c8_neighbours = c8.neighbours();
        assert_eq!(c8.alive(), true);
        assert_eq!(c8_neighbours, 3);

        let c9 = cell_array.cell(1, 3);
        let c9_neighbours = c9.neighbours();
        assert_eq!(c9.alive(), false);
        assert_eq!(c9_neighbours, 2);

        let c10 = cell_array.cell(1, 4);
        let c10_neighbours = c10.neighbours();
        assert_eq!(c10.alive(), false);
        assert_eq!(c10_neighbours, 1);

        //Third column
        let c11 = cell_array.cell(2, 0);
        let c11_neighbours = c11.neighbours();
        assert_eq!(c11.alive(), true);
        assert_eq!(c11_neighbours, 1);

        let c12 = cell_array.cell(2, 1);
        let c12_neighbours = c12.neighbours();
        assert_eq!(c12.alive(), true);
        assert_eq!(c12_neighbours, 3);

        let c13 = cell_array.cell(2, 2);
        let c13_neighbours = c13.neighbours();
        assert_eq!(c13.alive(), true);
        assert_eq!(c13_neighbours, 2);

        let c14 = cell_array.cell(2, 3);
        let c14_neighbours = c14.neighbours();
        assert_eq!(c14.alive(), false);
        assert_eq!(c14_neighbours, 2);

        let c15 = cell_array.cell(2, 4);
        let c15_neighbours = c15.neighbours();
        assert_eq!(c15.alive(), false);
        assert_eq!(c15_neighbours, 1);

        //Fourth column
        let c16 = cell_array.cell(3, 0);
        let c16_neighbours = c16.neighbours();
        assert_eq!(c16.alive(), false);
        assert_eq!(c16_neighbours, 2);

        let c17 = cell_array.cell(3, 1);
        let c17_neighbours = c17.neighbours();
        assert_eq!(c17.alive(), false);
        assert_eq!(c17_neighbours, 3);

        let c18 = cell_array.cell(3, 2);
        let c18_neighbours = c18.neighbours();
        assert_eq!(c18.alive(), false);
        assert_eq!(c18_neighbours, 2);

        let c19 = cell_array.cell(3, 3);
        let c19_neighbours = c19.neighbours();
        assert_eq!(c19.alive(), false);
        assert_eq!(c19_neighbours, 1);

        let c20 = cell_array.cell(3, 4);

        let c20_neighbours = c20.neighbours();
        assert_eq!(c20.alive(), false);
        assert_eq!(c20_neighbours, 1);

        //Fifth column
        let c21 = cell_array.cell(4, 0);
        let c21_neighbours = c21.neighbours();
        assert_eq!(c21.alive(), false);
        assert_eq!(c21_neighbours, 1);

        let c22 = cell_array.cell(4, 1);
        let c22_neighbours = c22.neighbours();
        assert_eq!(c22.alive(), false);
        assert_eq!(c22_neighbours, 1);

        let c23 = cell_array.cell(4, 2);
        let c23_neighbours = c23.neighbours();
        assert_eq!(c23.alive(), false);
        assert_eq!(c23_neighbours, 1);

        let c24 = cell_array.cell(4, 3);
        let c24_neighbours = c24.neighbours();
        assert_eq!(c24.alive(), false);
        assert_eq!(c24_neighbours, 0);

        let c25 = cell_array.cell(4, 4);
        let c25_neighbours = c25.neighbours();
        assert_eq!(c25.alive(), false);
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
        assert_eq!(cell.neighbour_cnt();, 8);
        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.to_string(), "00010001");
        assert_eq!(*cell == 0b00010001, true);
    } */
}
