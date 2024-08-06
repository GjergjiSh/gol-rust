//TODO: Remove
#![allow(warnings)]

mod gol;
use gol::*;
use std::{cell::RefCell, rc::Rc};

const H: usize = 100;
const W: usize = 100;
const GENERATIONS: usize = 100;
const SCALE: usize = 1;
const DELAY: usize = 20;

fn main() {
    let engine = Engine::<H, W>::new();
    let engine = RefCell::new(engine);
    engine.borrow_mut().randomize();

    let mut display = Display::<H, W>::new(EngineRef::new(&engine), DELAY);

    for _ in 0..GENERATIONS {
        engine.borrow_mut().generate();
        display.update();
    }

}
