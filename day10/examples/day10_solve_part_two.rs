use day10::part_two::solve;
fn main() {
    println!(
        "day10, part two result : {}",
        solve("day10/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day10::part_two::solve("input/part_two_input.txt"), 1186);
    }
}
