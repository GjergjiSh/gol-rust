use crate::gol::engine::{Engine, EngineRef};

use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time::Instant};

use minifb::{Key, Window, WindowOptions};

const COLOR_ALIVE: u32 = 0xFFFFFF; // White
const COLOR_DEAD: u32 = 0x000000; // Black
const SCALE: usize = 10; // Upscaling factor

//TODO: Display CFG
pub struct Display<'a, const H: usize, const W: usize> {
    engine: EngineRef<'a, H, W>,
    window: Window,
    delay: usize,
}

impl<'a, const H: usize, const W: usize> Display<'a, H, W> {
    pub fn new(engine: EngineRef<'a, H, W>, delay: usize) -> Self {
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
        for y in 0..H {
            for x in 0..W {
                let color = {
                    let engine = self.engine.borrow();
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

    pub fn run(&mut self, iterations: usize) {
        for _ in 0..iterations {
            if self.window.is_key_down(Key::Escape) {
                break;
            }
            self.update();
            std::thread::sleep(std::time::Duration::from_millis(self.delay as u64));
        }
    }
}

#[test]
fn test_display_time() {
    const H: usize = 100;
    const W: usize = 100;
    const GENERATIONS: usize = 100;
    const SCALE: usize = 100;
    const VERBOSE: bool = false;
    const DELAY: usize = 10;

    let engine = RefCell::new(Engine::<H, W>::new());
    engine.borrow_mut().randomize();

    //TODO: Builder pattern
    let mut display = Display::<H, W>::new(EngineRef::new(&engine), DELAY);

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