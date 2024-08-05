//TODO: Remove
#![allow(warnings)]

mod gol;
use gol::*;

const H: usize = 100;
const W: usize = 100;
const GENERATIONS: usize = 10;
const SCALE: usize = 10;
const VERBOSE: bool = false;

fn main() {
    let mut engine = Engine::<H, W>::new();
    let mut display = Display::<H, W>::new(&mut engine, 100);
    display.run(GENERATIONS);
}
