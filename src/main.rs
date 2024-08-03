//TODO: Remove
#![allow(warnings)]

mod cell;

use cell::CellArray;

/*
    Underpopulation: A live cell with fewer than 2 live neighbors dies.
    Survival: A live cell with 2 or 3 live neighbors lives on.
    Overpopulation: A live cell with more than 3 live neighbors dies.
    Reproduction: A dead cell with exactly 3 live neighbors becomes a live cell.
*/

fn count_neighbors(cell_array: &CellArray<3, 4>, x: isize, y: isize) -> u8 {
    let mut neighbour_count = 0;

    let neighbors = [
        cell_array.cell(x.wrapping_sub(1), y.wrapping_add(1)), // top_left
        cell_array.cell(x as isize, y.wrapping_add(1)),                 // top
        cell_array.cell(x.wrapping_add(1), y.wrapping_add(1)), // top_right
        cell_array.cell(x.wrapping_sub(1), y as isize),                 // left
        cell_array.cell(x.wrapping_add(1), y as isize),                 // right
        cell_array.cell(x.wrapping_sub(1), y.wrapping_sub(1)), // bottom_left
        cell_array.cell(x as isize, y.wrapping_sub(1)),                 // bottom
        cell_array.cell(x.wrapping_add(1), y.wrapping_sub(1)), // bottom_right
    ];

    for neighbor in neighbors.iter() {
        if neighbor.is_alive() {
            neighbour_count += 1;
        }
    }

    neighbour_count
}

fn solve(cell_array: &mut CellArray<3, 4>) {
    let rows = cell_array.rows();
    let cols = cell_array.cols();

    for x in 0..rows {
        for y in 0..cols {
            let neighbour_count = count_neighbors(cell_array, x as isize, y as isize);
            let cell = cell_array.mut_cell(x as isize, y as isize);
            let is_alive = cell.is_alive();

            cell.set_neighbors(neighbour_count);

            if is_alive {
                if neighbour_count < 2 || neighbour_count > 3 {
                    cell.kill();
                }
            } else {
                if neighbour_count == 3 {
                    cell.spawn();
                }
            }
        }
    }
}
fn main() {
    let mut cell_array = CellArray::<3, 4>::new();
    solve(&mut cell_array);
    println!("{}", cell_array);
}
