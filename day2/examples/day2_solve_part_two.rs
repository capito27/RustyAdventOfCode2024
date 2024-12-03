use day2::part_two::solve;
fn main() {
    println!(
        "day2, part two result : {}",
        solve("day2/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day2::part_two::solve("input/part_two_input.txt"), 290);
    }
}
