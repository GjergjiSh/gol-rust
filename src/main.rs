//TODO: Remove
#![allow(warnings)]

mod gol;
use gol::*;

const H: usize = 100;
const W: usize = 100;
const GENERATIONS: usize = 100;
const SCALE: usize = 100;
const VERBOSE: bool = false;
const DELAY: usize = 100;

//TODO: set_fps
//TODO: add shadow buffer around the edges and use that to calculate the next generation or rethink the wrapping

fn main() {
    let mut engine = Engine::<H, W>::new();
    let mut display = Display::<H, W>::new(&mut engine, DELAY);
    display.run(GENERATIONS);
}
