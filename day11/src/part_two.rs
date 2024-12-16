use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

pub fn iterate(state: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    state
        .iter()
        .flat_map(|(val, nbr)| {
            if *val == 0 {
                vec![(1, *nbr)]
            } else if val.ilog10() % 2 == 1 {
                let delim = 10u64.pow(val.ilog10() / 2 + 1);
                vec![(val % delim, *nbr), (val / delim, *nbr)]
            } else {
                vec![(*val * 2024, *nbr)]
            }
        })
        .fold(HashMap::new(), |mut acc, (key, val)| {
            *acc.entry(key).or_default() += val;
            acc
        })
}
pub fn solve<P>(input_file: P) -> u64
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let mut state: HashMap<u64, usize> = input
        .split(' ')
        .map(|number| (number.parse().unwrap(), 1))
        .collect();

    for i in 0..75 {
        state = iterate(&state);
    }
    state.into_values().sum::<usize>() as u64
}
