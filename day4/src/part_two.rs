use crate::_2DMask;
use std::fs::read_to_string;
use std::path::Path;

fn generate_xmas_pos_masks(w: isize) -> Vec<_2DMask> {
    // Valid masks shall resolve to MSASM
    let masks = vec![
        // M.S
        // .A.
        // M.S
        _2DMask::new(vec![0, 2, w + 1, 2 * w + 2, 2 * w], 0, 2),
        // S.S
        // .A.
        // M.M
        _2DMask::new(vec![2 * w, 2, w + 1, 0, 2 * w + 2], 0, 2),
        // S.M
        // .A.
        // S.M
        _2DMask::new(vec![2, 2 * w, w + 1, 0, 2 * w + 2], 0, 2),
        // M.M
        // .A.
        // S.S
        _2DMask::new(vec![0, 2 * w, w + 1, 2 * w + 2, 2], 0, 2),
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
            vals.len() == 5
                && *vals[0] == 'M'
                && *vals[1] == 'S'
                && *vals[2] == 'A'
                && *vals[3] == 'S'
                && *vals[4] == 'M'
        })
        .count() as u32
}
