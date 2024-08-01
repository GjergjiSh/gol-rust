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

    pub fn flip_bit(&mut self, idx: u8) {
        self.0 ^= 1 << idx;
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
