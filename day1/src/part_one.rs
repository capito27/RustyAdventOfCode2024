use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();

    let (left_heap, right_heap) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .fold(
            (BinaryHeap::new(), BinaryHeap::new()),
            |(mut acc_l, mut acc_r), (l, r)| {
                acc_l.push(l);
                acc_r.push(r);
                (acc_l, acc_r)
            },
        );

    left_heap
        .into_sorted_vec()
        .iter()
        .zip(right_heap.into_sorted_vec().iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}
