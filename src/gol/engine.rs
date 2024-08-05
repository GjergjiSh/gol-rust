use crate::gol::types::*;
pub struct Engine<const H: usize, const W: usize> {
    cells: CellArray<H, W>,
    cell_cache: CellArray<H, W>,
}

impl<const H: usize, const W: usize> Engine<H, W> {
    pub fn new() -> Self {
        Self {
            cells: CellArray::new(),
            cell_cache: CellArray::new(),
        }
    }

    pub fn spawn_glider(&mut self, x: isize, y: isize) {
        let pattern_coords = [
            (x + 2, y),
            (x + 2, y + 1),
            (x + 2, y + 2),
            (x + 1, y + 2),
            (x, y + 1),
        ];

        for &(x, y) in &pattern_coords {
            self.cells.spawn(x, y)
        }
    }

    pub fn randomize(&mut self) {
        for x in 0..H {
            for y in 0..W {
                if rand::random() {
                    self.cells.spawn(x as isize, y as isize);
                }
            }
        }
    }

    pub fn generate(&mut self) {
        let rows = self.cells.rows();
        let cols = self.cells.cols();

        //TODO: Optimize this
        self.cell_cache.clone_from(&self.cells);
        for x in 0..rows {
            for y in 0..cols {
                let cell = self.cell_cache.mut_cell(x as isize, y as isize);
                let neighbour_count = cell.neighbour_cnt();

                if cell.alive() {
                    if neighbour_count < 2 || neighbour_count > 3 {
                        self.cells.kill_cell(x as isize, y as isize);
                    }
                } else {
                    if neighbour_count == 3 {
                        self.cells.spawn(x as isize, y as isize);
                    }
                }
            }
        }
    }

    pub fn cells(&self) -> &CellArray<H, W> {
        &self.cells
    }

    pub fn cell(&self, x: isize, y: isize) -> &Cell {
        self.cells.cell(x, y)
    }

    pub fn print(&self) {
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
                let cell = self.cells.cell(x as isize, y as isize);
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
