mod gol;
use gol::*;
use std::cell::RefCell;

const H: usize = 1000;
const W: usize = 1000;
const GENERATIONS: usize = 1000;
const DELAY: usize = 20;

fn main() {
    let engine = RefCell::new(Engine::<H, W>::new());
    let mut display = Display::<H, W>::new(&engine, DELAY);
    engine.borrow_mut().randomize();


    for _ in 0..GENERATIONS {
        engine.borrow_mut().generate();
        display.update();
    }

}
