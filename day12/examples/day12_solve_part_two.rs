use day12::part_two::solve;
fn main() {
    println!(
        "day12, part two result : {}",
        solve("day12/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day12::part_two::solve("input/part_two_input.txt"), 0);
    }
}
