use day9::part_two::solve;
fn main() {
    println!(
        "day9, part two result : {}",
        solve("day9/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(
            day9::part_two::solve("input/part_two_input.txt"),
            6360363199987
        );
    }
}
