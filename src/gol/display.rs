use crate::gol::engine::{Engine /* EngineRef */};

use std::cell::RefCell;

use minifb::{Window, WindowOptions};

const COLOR_ALIVE: u32 = 0xFFFFFF; // White
const COLOR_DEAD: u32 = 0x000000; // Black
const SCALE: usize = 10; // Upscaling factor

pub struct Display<'a, const H: usize, const W: usize> {
    engine: &'a RefCell<Engine<H, W>>,
    window: Window,
    delay: usize,
}

impl<'a, const H: usize, const W: usize> Display<'a, H, W> {
    pub fn new(engine: &'a RefCell<Engine<H, W>>, delay: usize) -> Self {
        let window = Window::new(
            "Conway's Game of Life",
            W * SCALE,
            H * SCALE,
            WindowOptions::default(),
        )
        .unwrap();

        Self {
            engine,
            window,
            delay,
        }
    }

    pub fn update(&mut self) {
        let mut buffer: Vec<u32> = vec![0; W * H];
        let engine = self.engine.borrow();

        for y in 0..H {
            for x in 0..W {
                let color = {
                    let cell = engine.cells().cell(x as isize, y as isize);
                    if cell.alive() {
                        COLOR_ALIVE
                    } else {
                        COLOR_DEAD
                    }
                };
                buffer[y * W + x] = color;
            }
        }
        self.window.update_with_buffer(&buffer, W, H).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(self.delay as u64));
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Instant;

    #[test]
    fn test_display_time() {
        const H: usize = 100;
        const W: usize = 100;
        const GENERATIONS: usize = 100;
        const DELAY: usize = 10;

        let engine = RefCell::new(Engine::<H, W>::new());
        engine.borrow_mut().randomize();

        //TODO: Builder pattern
        let mut display = Display::<H, W>::new(&engine, DELAY);

        let start = Instant::now();
        for _ in 0..GENERATIONS {
            engine.borrow_mut().generate();
            display.update();
        }
        let duration = start.elapsed();

        println!(
            "Display::update() took {} milliseconds",
            duration.as_millis()
        );
    }
}
