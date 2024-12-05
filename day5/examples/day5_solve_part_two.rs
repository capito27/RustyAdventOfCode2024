use day5::part_two::solve;
fn main() {
    println!(
        "day5, part two result : {}",
        solve("day5/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day5::part_two::solve("input/part_two_input.txt"), 6257);
    }
}
