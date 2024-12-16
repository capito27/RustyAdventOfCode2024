use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u64
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let mut expanded = input
        .trim()
        .char_indices()
        .flat_map(|(idx, c)| {
            if idx % 2 == 1 { [-1] } else { [idx as i64 / 2] }
                .repeat(c.to_digit(10).unwrap() as usize)
        })
        .collect::<Vec<i64>>();

    let (mut left, mut right) = (0usize, expanded.len() - 1);
    while left < right {
        while expanded[right] == -1 && left < right {
            right -= 1;
        }

        while expanded[left] != -1 && left < right {
            left += 1;
        }
        if left < right {
            expanded.swap(left, right);
        }
    }

    expanded
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != -1)
        .map(|(idx, id)| *id as u64 * idx as u64)
        .sum()
}
