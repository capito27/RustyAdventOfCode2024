use day7::part_two::solve;
fn main() {
    println!(
        "day7, part two result : {}",
        solve("day7/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(
            day7::part_two::solve("input/part_two_input.txt"),
            97902809384118
        );
    }
}
