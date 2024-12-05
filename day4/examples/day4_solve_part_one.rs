use day4::part_one::solve;
fn main() {
    println!(
        "day4, part one result : {}",
        solve("day4/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day4::part_one::solve("input/part_one_input.txt"), 2639);
    }
}
