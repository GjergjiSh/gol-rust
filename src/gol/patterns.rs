use crate::gol::types::CellArray;

pub fn spawn_glider<const H: usize, const W: usize>(cell_array: &mut CellArray<H, W>, x: isize, y: isize) {
    let pattern_coords = [
        (x + 2, y),
        (x + 2, y + 1),
        (x + 2, y + 2),
        (x + 1, y + 2),
        (x, y + 1),
    ];

    for &(x, y) in &pattern_coords {
        cell_array.spawn(x, y)
    }
}