use crate::{GridElement, Map};
use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let mut map = Map::new(&input);
    while map.iterate() {}

    println!("{map:?}");
    map.grid
        .iter()
        .map(|v| {
            v.iter()
                .filter(|g| matches!(g, GridElement::Visited(_)))
                .count() as u32
        })
        .sum()
}
