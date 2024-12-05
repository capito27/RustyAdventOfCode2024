use std::cmp::Ordering;
use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let rules = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("|").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    updates
        .into_iter()
        .filter(|pages| {
            // only care about updates that are NOT already sorted based on the rules,
            // so check if we break any rules, and invert
            !pages.is_sorted_by(|a, b| !rules.contains(&(*b, *a)))
        })
        .map(|mut update| {
            update.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Greater
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|pages| pages[pages.len() / 2])
        .sum()
}
