pub mod part_one;
pub mod part_two;

pub fn is_line_valid(line: &Vec<u32>) -> bool {
    line.windows(2)
        .all(|w| 1 <= w[0].abs_diff(w[1]) && w[0].abs_diff(w[1]) <= 3)
        && (line.is_sorted() || line.iter().rev().is_sorted())
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_test_part_one() {
        use crate::part_one::solve;
        assert_eq!(solve("input/part_one_sample.txt"), 2);
    }

    #[test]
    fn sanity_test_part_two() {
        use crate::part_two::solve;
        assert_eq!(solve("input/part_two_sample.txt"), 4);
    }
}
