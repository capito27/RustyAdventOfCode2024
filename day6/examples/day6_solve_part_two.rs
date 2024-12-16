use day6::part_two::solve;
fn main() {
    println!(
        "day6, part two result : {}",
        solve("day6/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day6::part_two::solve("input/part_two_input.txt"), 2008);
    }
}