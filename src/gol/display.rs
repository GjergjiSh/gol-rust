use minifb::{Key, Window, WindowOptions};

use crate::gol::engine::Engine;

const COLOR_ALIVE: u32 = 0xFFFFFF; // White
const COLOR_DEAD: u32 = 0x000000; // Black
const SCALE: usize = 10; // Upscaling factor

pub struct Display<'a, const H: usize, const W: usize> {
    engine: &'a mut Engine<H, W>,
    delay: usize,
}

impl<'a, const H: usize, const W: usize> Display<'a, H, W> {
    pub fn new(engine: &'a mut Engine<H, W>, delay: usize) -> Self {
        //TODO: Remove this
        engine.spawn_glider(3, 3);
        Self {
            engine: engine,
            delay: delay,
        }
    }

    pub fn run(&mut self, iterations: usize) {
        let mut window = Window::new(
            "Conway's Game of Life",
            W * SCALE,
            H * SCALE,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.engine.generate(iterations);
            let mut buffer: Vec<u32> = vec![0; W * H];
            for y in 0..H {
                for x in 0..W {
                    let cell = self.engine.cells().cell(x as isize, y as isize);
                    let color = if cell.alive() { 0xFFFFFF } else { 0x000000 }; // White for alive, black for dead
                    buffer[y * W + x] = color;
                }
            }
            window.update_with_buffer(&buffer, W, H).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(self.delay as u64));
        }
    }
}
