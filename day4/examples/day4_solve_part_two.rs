use day4::part_two::solve;
fn main() {
    println!(
        "day4, part two result : {}",
        solve("day4/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day4::part_two::solve("input/part_two_input.txt"), 2005);
    }
}
