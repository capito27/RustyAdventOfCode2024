use day5::part_one::solve;
fn main() {
    println!(
        "day5, part one result : {}",
        solve("day5/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day5::part_one::solve("input/part_one_input.txt"), 4766);
    }
}
