use crate::is_line_valid;
use std::fs::read_to_string;
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
        .filter(is_line_valid)
        .count() as u32
}
