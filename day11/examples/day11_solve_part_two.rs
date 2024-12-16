use day11::part_two::solve;
fn main() {
    println!(
        "day11, part two result : {}",
        solve("day11/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(
            day11::part_two::solve("input/part_two_input.txt"),
            216318908621637
        );
    }
}
