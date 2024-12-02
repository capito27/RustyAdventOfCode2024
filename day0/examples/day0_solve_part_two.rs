use day0::part_two::solve;
fn main() {
    println!(
        "day0, part two result : {}",
        solve("day0/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day0::part_two::solve("input/part_two_input.txt"), 0);
    }
}
