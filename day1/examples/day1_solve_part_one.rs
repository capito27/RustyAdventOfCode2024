use day1::part_one::solve;
fn main() {
    println!(
        "day1, part one result : {}",
        solve("day1/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day1::part_one::solve("input/part_one_input.txt"), 1603498);
    }
}
