use day1::part_two::solve;
fn main() {
    println!(
        "day1, part two result : {}",
        solve("day1/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day1::part_two::solve("input/part_two_input.txt"), 25574739);
    }
}
