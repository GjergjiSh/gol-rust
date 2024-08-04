//TODO: Remove
#![allow(warnings)]

mod gol;
use gol::cell_array::CellArray;
use gol::cell::Cell;
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



fn count_neighbours(cell_array: &CellArray<ARRAY_H, ARRAY_W>, x: isize, y: isize) -> u8 {
    let mut neighbour_count = 0;

    let neighbors = [
        cell_array.cell(x.wrapping_sub(1), y.wrapping_sub(1)), // top_left
        cell_array.cell(x, y.wrapping_sub(1)),                 // top
        cell_array.cell(x.wrapping_add(1), y.wrapping_sub(1)), // top_right
        cell_array.cell(x.wrapping_sub(1), y),                 // left
        cell_array.cell(x.wrapping_add(1), y),                 // right
        cell_array.cell(x.wrapping_sub(1), y.wrapping_add(1)), // bottom_left
        cell_array.cell(x, y.wrapping_add(1)),                 // bottom
        cell_array.cell(x.wrapping_add(1), y.wrapping_add(1)), // bottom_right
    ];

    for neighbor in neighbors.iter() {
        if neighbor.alive() {
            neighbour_count += 1;
        }
    }

    neighbour_count
}

fn solve(
    cell_array: &mut CellArray<ARRAY_H, ARRAY_W>,
    cell_array_copy: &mut CellArray<ARRAY_H, ARRAY_W>,
) {
    let rows = cell_array.width();
    let cols = cell_array.height();

    cell_array_copy.clone_from(&cell_array);

    for x in 0..rows - 1 {
        for y in 0..cols - 1 {
            let cell = cell_array_copy.mut_cell(x as isize, y as isize);
            let is_alive = cell.alive();
            let neighbour_count = cell.neighbour_cnt();
            if VERBOSE {
                println!(
                    "Cell at ({}, {}) has {} neighbours and is {}",
                    x, y, neighbour_count, if is_alive { "alive" } else { "dead" }
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

    for x in 0..cell_array.width() - 1 {
        for y in 0..cell_array.height() - 1 {
            let neighbour_count = count_neighbours(&cell_array, x as isize, y as isize);
            let cell = cell_array.mut_cell(x as isize, y as isize);
            // cell.set_neighbors(neighbour_count);
        }
    }

    cell_array.print();
    // println!("{}", cell_array);
    {
        let c1 = cell_array.cell(3, 4);
        let c1_cnt = count_neighbours(&cell_array, 3, 4);
        assert_eq!(c1_cnt, 1);
        assert_eq!(*c1, 0b00000011);
        assert_eq!(c1.to_string(), "00000011");
        let c2 = cell_array.cell(4, 5);
        let c2_cnt = count_neighbours(&cell_array, 4, 5);
        assert_eq!(c2_cnt, 3);
        assert_eq!(*c2, 0b00000111);
        assert_eq!(c2.to_string(), "00000111");
        let c3 = cell_array.cell(5, 3);
        let c3_cnt = count_neighbours(&cell_array, 5, 3);
        assert_eq!(c3_cnt, 1);
        assert_eq!(*c3, 0b00000011);
        assert_eq!(c3.to_string(), "00000011");
        let c4 = cell_array.cell(5, 4);
        let c4_cnt = count_neighbours(&cell_array, 5, 4);
        assert_eq!(c4_cnt, 3);
        assert_eq!(*c4, 0b00000111);
        assert_eq!(c4.to_string(), "00000111");
        let c5 = cell_array.cell(5, 5);
        let c5_cnt = count_neighbours(&cell_array, 5, 5);
        assert_eq!(c5_cnt, 2);
        assert_eq!(*c5, 0b00000101);
    }

    //Shallow copy
    let mut temp = cell_array;

    for x in 0..4 {
        println!("Iteration {}", x);
        solve(&mut cell_array, &mut temp);
        cell_array.print();
        println!();
        // println!("{}", cell_array);
    }
}
