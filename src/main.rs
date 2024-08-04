//TODO: Remove
#![allow(warnings)]

mod cell;

use cell::CellArray;
use minifb::{Key, Window, WindowOptions};

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

fn spawn_glider(
    cell_array: &mut CellArray<ARRAY_H, ARRAY_W>,
    top_left_x: isize,
    top_left_y: isize,
) {
    let glider_coords = [
        (top_left_x + 1, top_left_y),     // (1, 0)
        (top_left_x + 2, top_left_y + 1), // (2, 1)
        (top_left_x, top_left_y + 2),     // (0, 2)
        (top_left_x + 1, top_left_y + 2), // (1, 2)
        (top_left_x + 2, top_left_y + 2), // (2, 2)
    ];

    for &(x, y) in &glider_coords {
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();
    }
}

fn spawn_custom_pattern(cell_array: &mut CellArray<ARRAY_H, ARRAY_W>, x: isize, y: isize) {
    let pattern_coords = [
        (x + 2, y),
        (x + 2, y + 1),
        (x + 2, y + 2),
        (x + 1, y + 2),
        (x, y + 1),
    ];

    for &(x, y) in &pattern_coords {
        let neighbour_count = count_neighbours(cell_array, x, y);
        let cell = cell_array.mut_cell(x, y);
        cell.spawn();
    }
}

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
        if neighbor.is_alive() {
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
            let is_alive = cell.is_alive();
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

fn render_field(cell_array: &CellArray<ARRAY_H, ARRAY_W>, buffer: &mut Vec<u32>) {
    for y in 0..ARRAY_H {
        for x in 0..ARRAY_W {
            let cell = cell_array.cell(x as isize, y as isize);
            let color = if cell.is_alive() { 0x000000 } else { 0xFFFFFF };
            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    buffer[(y * SCALE + dy) * ARRAY_W * SCALE + (x * SCALE + dx)] = color;
                }
            }
        }
    }
}

fn render_field_console(cell_array: &CellArray<ARRAY_H, ARRAY_W>) {
    // Print the top border with column indices
    print!("   "); // Space for row indices
    println!();

    // Print the top border of the grid with column numbers
    print!("  +");
    for x in 0..ARRAY_W {
        print!("-{}-+", x);
    }
    println!();

    // Print the field with side borders and row indices
    for y in 0..ARRAY_H {
        print!("{:2}|", y); // Row index
        for x in 0..ARRAY_W {
            let cell = cell_array.cell(x as isize, y as isize);
            let symbol = if cell.is_alive() { '*' } else { ' ' };
            print!(" {} |", symbol);
        }
        println!(); // End of the row with a side border

        // Print the horizontal border between rows without column numbers
        print!("  +");
        for _ in 0..ARRAY_W {
            print!("---+");
        }
        println!();
    }
}

fn main() {
    let mut cell_array = CellArray::<ARRAY_H, ARRAY_W>::new();
    spawn_custom_pattern(&mut cell_array, 3, 3);

    for x in 0..cell_array.width() - 1 {
        for y in 0..cell_array.height() - 1 {
            let neighbour_count = count_neighbours(&cell_array, x as isize, y as isize);
            let cell = cell_array.mut_cell(x as isize, y as isize);
            cell.set_neighbors(neighbour_count);
           /*  if cell.is_alive() {
                println!(
                    "Cell at ({}, {}) has {} neighbours and is alive",
                    x, y, neighbour_count
                );
            } */
        }
    }

    render_field_console(&cell_array);
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
        render_field_console(&cell_array);
        println!();
        // println!("{}", cell_array);
    }

    // let mut window = Window::new(
    //     "Game of Life",
    //     ARRAY_W * SCALE,
    //     ARRAY_H * SCALE,
    //     WindowOptions::default(),
    // )
    // .unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });

    // let mut buffer: Vec<u32> = vec![0; ARRAY_W * ARRAY_H * SCALE * SCALE];

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     solve(&mut cell_array);
    //     render_field(&cell_array, &mut buffer);
    //     window.update_with_buffer(&buffer, ARRAY_W * SCALE, ARRAY_H * SCALE).unwrap();
    // }
}
