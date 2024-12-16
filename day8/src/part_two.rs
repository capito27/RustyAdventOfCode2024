use crate::Point2D;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let (max_y, max_x) = (
        input.lines().count() as i32,
        input.find('\n').unwrap() as i32 - 1,
    );
    let grouped_antennas: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().flat_map(move |(x, c)| {
                if c != '.' {
                    Some((c, (x as i32, y as i32).into()))
                } else {
                    None
                }
            })
        })
        .chunk_by(|(key, _)| *key)
        .into_iter()
        .fold(
            HashMap::new(),
            |mut res: HashMap<char, Vec<Point2D>>, (key, run)| {
                res.entry(key).or_default().extend(run.map(|(_, p)| p));
                res
            },
        )
        .into_values()
        .collect();
    grouped_antennas
        .iter()
        .flat_map(|antenna| {
            antenna.iter().combinations(2).flat_map(|combinations| {
                let mut res = combinations[0].extrapolate_n(combinations[1], 0, max_x, 0, max_y);
                res.extend(combinations[1].extrapolate_n(combinations[0], 0, max_x, 0, max_y));
                res
            })
        })
        .filter(|p| p.x >= 0 && p.y >= 0 && p.x < max_x && p.y < max_y)
        .unique()
        .count() as u32
}
