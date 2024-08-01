//TODO: Remove
#![allow(warnings)]

mod field;

use field::Field;

fn main() {
    let mut field = Field::<2, 5>::new();

    let cell = field.mut_cell(0, 0);
    cell.spawn();
    assert_eq!(cell.is_alive(), true);
    assert_eq!(cell.to_string(), "00000001");
    assert_eq!(*cell == 0b00000001, true);

    let cell = field.mut_cell(0, 0);
    cell.flip_bit(0);
    assert_eq!(cell.is_alive(), false);
    assert_eq!(cell.to_string(), "00000000");
    assert_eq!(*cell == 0b00000000, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(0);
    assert_eq!(cell.neighbour_cnt(), 0);
    assert_eq!(cell.to_string(), "00000000");
    assert_eq!(*cell == 0b00000000, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(1);
    assert_eq!(cell.neighbour_cnt(), 1);
    assert_eq!(cell.to_string(), "00000010");
    assert_eq!(*cell == 0b00000010, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(2);
    assert_eq!(cell.neighbour_cnt(), 2);
    assert_eq!(cell.to_string(), "00000100");
    assert_eq!(*cell == 0b00000100, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(3);
    assert_eq!(cell.neighbour_cnt(), 3);
    assert_eq!(cell.to_string(), "00000110");
    assert_eq!(*cell == 0b00000110, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(4);
    assert_eq!(cell.neighbour_cnt(), 4);
    assert_eq!(cell.to_string(), "00001000");
    assert_eq!(*cell == 0b00001000, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(5);
    assert_eq!(cell.neighbour_cnt(), 5);
    assert_eq!(cell.to_string(), "00001010");
    assert_eq!(*cell == 0b00001010, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(6);
    assert_eq!(cell.neighbour_cnt(), 6);
    assert_eq!(cell.to_string(), "00001100");
    assert_eq!(*cell == 0b00001100, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(7);
    assert_eq!(cell.neighbour_cnt(), 7);
    assert_eq!(cell.to_string(), "00001110");
    assert_eq!(*cell == 0b00001110, true);

    let cell = field.mut_cell(0, 0);
    cell.set_neighbors(8);
    assert_eq!(cell.neighbour_cnt(), 8);
    assert_eq!(cell.to_string(), "00010000");
    assert_eq!(*cell == 0b00010000, true);

    println!("{}", field);
}
