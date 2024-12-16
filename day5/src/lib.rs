pub mod part_one;
pub mod part_two;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";
#[cfg(test)]
mod tests {
    #[test]
    fn sanity_test_part_one() {
        use crate::part_one::solve;
        assert_eq!(solve("input/part_one_sample.txt"), 143);
    }

    #[test]
    fn sanity_test_part_two() {
        use crate::part_two::solve;
        assert_eq!(solve("input/part_two_sample.txt"), 123);
    }
}
