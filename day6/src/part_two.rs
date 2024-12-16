use crate::{GridElement, Map};
use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let map = Map::new(&input);
    (0..map.grid.len())
        .map(|x| {
            (0..map.grid[0].len())
                .filter({
                    let grid = map.grid.clone();
                    move |y| {
                        !(matches!(grid[x][*y], GridElement::Obstacle)
                            || (x, *y).eq(&map.guard_position))
                    }
                })
                .filter({
                    let map = map.clone();
                    move |y| {
                        let mut map = map.clone();
                        map.grid[x][*y] = GridElement::Obstacle;
                        while map.iterate() {}
                        map.loops
                    }
                })
                .count() as u32
        })
        .sum()
}
