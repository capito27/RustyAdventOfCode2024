use crate::_2DMask;
use std::fs::read_to_string;
use std::path::Path;

fn generate_xmas_pos_masks(w: isize) -> Vec<_2DMask> {
    let masks = vec![
        // add all masks based on clockwise rotation from horizontal, left to right
        // →
        _2DMask::new(vec![0, 1, 2, 3], 0, 3),
        // ↘
        _2DMask::new(vec![0, w + 1, 2 * (w + 1), 3 * (w + 1)], 0, 3),
        // ↓
        _2DMask::new(vec![0, w, 2 * w, 3 * w], 0, 0),
        // ↙
        _2DMask::new(vec![0, w - 1, 2 * (w - 1), 3 * (w - 1)], 3, 0),
        // ←
        _2DMask::new(vec![0, -1, -2, -3], 3, 0),
        // ↖
        _2DMask::new(vec![0, -w - 1, 2 * (-w - 1), 3 * (-w - 1)], 3, 0),
        // ↑
        _2DMask::new(vec![0, -w, -2 * w, -3 * w], 0, 0),
        // ↗
        _2DMask::new(vec![0, -w + 1, -2 * w + 2, -3 * w + 3], 0, 3),
    ];

    masks
}
pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let width = input.find("\n").unwrap() as isize;
    let input = input.replace("\n", "").chars().collect::<Vec<char>>();
    let input_slice = input.as_slice();
    let masks = generate_xmas_pos_masks(width);
    (0..input.len() as isize)
        .flat_map(|pos| {
            masks
                .iter()
                .filter_map(move |mask| mask.apply(pos, width, input_slice))
        })
        .filter(|vals| {
            vals.len() == 4
                && *vals[0] == 'X'
                && *vals[1] == 'M'
                && *vals[2] == 'A'
                && *vals[3] == 'S'
        })
        .count() as u32
}
