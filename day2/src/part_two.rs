use crate::is_line_valid;
use std::fs::read_to_string;
use std::iter::Iterator;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|line| {
            is_line_valid(line)
                || (0..line.len()).any(|skipped| {
                    let new_line = line
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, val)| if idx != skipped { Some(*val) } else { None })
                        .collect::<Vec<u32>>();
                    is_line_valid(&new_line)
                })
        })
        .count() as u32
}
