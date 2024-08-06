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

}

/* pub struct EngineRef<'a, const H: usize, const W: usize> {
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
} */

/* pub struct ImmutableEngineRef<'a, const H: usize, const W: usize> {
    engine: &'a RefCell<Engine<H, W>>,
}

impl<'a, const H: usize, const W: usize> ImmutableEngineRef<'a, H, W> {
    pub fn new(engine: &'a RefCell<Engine<H, W>>) -> Self {
        Self { engine }
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, Engine<H, W>> {
        self.engine.borrow()
    }
} */

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
