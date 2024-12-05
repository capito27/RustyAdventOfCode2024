use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    updates
        .into_iter()
        .filter(|pages| {
            // only care about updates that are already sorted based on the rules,
            // so check if we break any rules
            pages.is_sorted_by(|a, b| !rules.contains(&(*b, *a)))
        })
        .map(|pages| pages[pages.len() / 2])
        .sum()
}
