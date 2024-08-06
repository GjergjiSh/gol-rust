use std::{cell::RefCell, rc::Rc};

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
        //TODO: Optimize this
        self.cell_cache.clone_from(&self.cells);

        for x in 0..self.cells.rows() {
            for y in 0..self.cells.cols() {
                let cell = self.cell_cache.mut_cell(x as isize, y as isize);

                if *cell == 0b00000000 {
                    continue;
                }

                let neighbour_count = cell.neighbours();

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

// Immutable reference to the Engine
pub struct EngineRef<'a, const H: usize, const W: usize> {
    engine: &'a RefCell<Engine<H, W>>,
}

impl<'a, const H: usize, const W: usize> EngineRef<'a, H, W> {
    pub fn new(engine: &'a RefCell<Engine<H, W>>) -> Self {
        Self { engine }
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, Engine<H, W>> {
        self.engine.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, Engine<H, W>> {
        self.engine.borrow_mut()
    }
}

pub struct ImmutableEngineRef<'a, const H: usize, const W: usize> {
    engine: &'a RefCell<Engine<H, W>>,
}

impl<'a, const H: usize, const W: usize> ImmutableEngineRef<'a, H, W> {
    pub fn new(engine: &'a RefCell<Engine<H, W>>) -> Self {
        Self { engine }
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, Engine<H, W>> {
        self.engine.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        thread,
        time::{Duration, Instant},
    };

    fn measure_execution_time<F>(mut f: F, count: usize) -> (Duration, Duration)
    where
        F: FnMut(),
    {
        let mut total_time = Duration::new(0, 0);

        for _ in 0..count {
            let start = Instant::now();
            f();
            let duration = start.elapsed();
            total_time += duration;
        }

        let average_time = total_time / count as u32;
        (average_time, total_time)
    }

    fn copy<const H: usize, const W: usize>(src: &CellArray<H, W>, dst: &mut CellArray<H, W>) {
        let src_cells = src.cells();
        let dst_cells = dst.cells();

        // dest_cells.copy_from_slice(src_cells);
    }

    #[test]
    fn test_clone_time() {
        const H: usize = 100;
        const W: usize = 100;
        const COUNT: usize = 1000;
        let size = H * W;
        let mut engine = Engine::<H, W>::new();
        let (average_time, total_time) =
            measure_execution_time(|| engine.cell_cache.clone_from(&engine.cells), COUNT);

        println!(
            "Average time taken to clone {} bytes once: {:?}",
            size, average_time
        );
        println!(
            "Total time taken to clone {} bytes {} times: {:?}",
            size, COUNT, total_time
        );
    }

    #[test]
    fn test_memcopy_time() {
        // const H: usize = 100;
        // const W: usize = 100;
        // const COUNT: usize = 1000;
        // let mut engine = Engine::<H, W>::new();
        // let (average_time, total_time) = measure_execution_time(
        //     || engine.cell_cache.memcopy_from(&engine.cells),
        //     COUNT,
        // );

        // println!("Average time taken to memcopy: {:?}", average_time);
        // println!("Total time taken to memcopy: {:?}", total_time);
    }

    #[test]
    fn test_generate_time() {
        let stack_size = 100 * 1024 * 1024; // 100 MB

        let handler = thread::Builder::new()
            .stack_size(stack_size)
            .spawn(|| {
                const H: usize = 1000;
                const W: usize = 1000;
                let mut engine = Engine::<H, W>::new();

                let start = std::time::Instant::now();
                engine.generate();
                let end = std::time::Instant::now();
                println!("Time taken to generate: {:?}", end.duration_since(start));
            })
            .unwrap();

        handler.join().unwrap();
    }
}
