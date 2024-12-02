use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Mul;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let (left, right) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut vec, mut hashmap), (l, r)| {
                vec.push(l);
                hashmap.insert(r, hashmap.get(&r).unwrap_or(&0u32) + 1);
                (vec, hashmap)
            },
        );
    left.iter().map(|l| l.mul(right.get(l).unwrap_or(&0))).sum()
}
