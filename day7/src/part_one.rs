use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::Path;

pub fn is_sum_reachable_by_digits(sum: u64, digits: &[u64]) -> bool {
    let mut deque: VecDeque<u64> = VecDeque::new();
    deque.push_back(digits[0]);
    digits.iter().enumerate().skip(1).for_each(|(i, &digit)| {
        for j in 0..2usize.pow(i as u32) {
            let front = deque.front().unwrap();
            if j % 2 == 1 {
                deque.push_back(front * digit);
                deque.pop_front();
            } else {
                deque.push_back(front + digit);
            }
        }
    });

    deque.contains(&sum)
}

pub fn solve<P>(input_file: P) -> u64
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    input
        .lines()
        .filter_map(|l| {
            let (sum, digits) = l.split_once(": ").unwrap();
            let sum = sum.parse::<u64>().unwrap();
            let numbers = digits
                .split(' ')
                .flat_map(|nbr| nbr.parse::<u64>())
                .collect::<Vec<_>>();
            if is_sum_reachable_by_digits(sum, &numbers) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}
