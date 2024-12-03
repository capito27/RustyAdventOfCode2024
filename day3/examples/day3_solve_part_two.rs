use day3::part_two::solve;
fn main() {
    println!(
        "day3, part two result : {}",
        solve("day3/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day3::part_two::solve("input/part_two_input.txt"), 92626942);
    }
}
