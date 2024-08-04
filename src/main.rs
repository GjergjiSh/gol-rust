//TODO: Remove
#![allow(warnings)]

mod gol;
use gol::cell::Cell;
use gol::cell_array::CellArray;
use gol::utils::spawn_glider;

const ARRAY_H: usize = 10;
const ARRAY_W: usize = 10;
const SCALE: usize = 10;
const VERBOSE: bool = false;

/*
    Underpopulation: A live cell with fewer than 2 live neighbors dies.
    Survival: A live cell with 2 or 3 live neighbors lives on.
    Overpopulation: A live cell with more than 3 live neighbors dies.
    Reproduction: A dead cell with exactly 3 live neighbors becomes a live cell.
*/

fn solve(
    cell_array: &mut CellArray<ARRAY_H, ARRAY_W>,
    cell_array_copy: &mut CellArray<ARRAY_H, ARRAY_W>,
) {
    let rows = cell_array.width();
    let cols = cell_array.height();

    cell_array_copy.clone_from(&cell_array);

    for x in 0..rows {
        for y in 0..cols {
            let cell = cell_array_copy.mut_cell(x as isize, y as isize);
            let is_alive = cell.alive();
            let neighbour_count = cell.neighbour_cnt();
            if VERBOSE {
                println!(
                    "Cell at ({}, {}) has {} neighbours and is {}",
                    x,
                    y,
                    neighbour_count,
                    if is_alive { "alive" } else { "dead" }
                );
            }

            if is_alive {
                if neighbour_count < 2 || neighbour_count > 3 {
                    if VERBOSE {
                        println!("Killing cell at ({}, {})", x, y);
                    }
                    cell_array.kill_cell(x as isize, y as isize);
                }
            } else {
                if neighbour_count == 3 {
                    if VERBOSE {
                        println!("Spawning cell at ({}, {})", x, y);
                    }
                    cell_array.spawn_cell(x as isize, y as isize);
                }
            }
        }
    }
}

fn main() {
    let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();
    spawn_glider(&mut cell_array, 3, 3);
    //Shallow copy
    let mut temp = cell_array;

    cell_array.print();

    {
        let c1 = cell_array.cell(3, 4);
        let c1_cnt = c1.neighbour_cnt();
        assert_eq!(c1_cnt, 1);
        assert_eq!(*c1, 0b00000011);
        assert_eq!(c1.to_string(), "00000011");
        let c2 = cell_array.cell(4, 5);
        let c2_cnt = c2.neighbour_cnt();
        assert_eq!(c2_cnt, 3);
        assert_eq!(*c2, 0b00000111);
        assert_eq!(c2.to_string(), "00000111");
        let c3 = cell_array.cell(5, 3);
        let c3_cnt = c3.neighbour_cnt();
        assert_eq!(c3_cnt, 1);
        assert_eq!(*c3, 0b00000011);
        assert_eq!(c3.to_string(), "00000011");
        let c4 = cell_array.cell(5, 4);
        let c4_cnt = c4.neighbour_cnt();
        assert_eq!(c4_cnt, 3);
        assert_eq!(*c4, 0b00000111);
        assert_eq!(c4.to_string(), "00000111");
        let c5 = cell_array.cell(5, 5);
        let c5_cnt = c5.neighbour_cnt();
        assert_eq!(c5_cnt, 2);
        assert_eq!(*c5, 0b00000101);
    }

    for x in 0..4 {
        solve(&mut cell_array, &mut temp);
        cell_array.print();
        let cell = cell_array.cell(5, 3);
    }
}
