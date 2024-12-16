use day8::part_two::solve;
fn main() {
    println!(
        "day8, part two result : {}",
        solve("day8/input/part_two_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day8::part_two::solve("input/part_two_input.txt"), 1064);
    }
}
