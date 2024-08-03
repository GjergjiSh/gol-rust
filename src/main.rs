//TODO: Remove
#![allow(warnings)]

mod field;

use field::Field;

fn main() {
    let mut field = Field::<2, 5>::new();
    println!("{}", field);
}
