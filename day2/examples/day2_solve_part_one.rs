use day2::part_one::solve;
fn main() {
    println!(
        "day2, part one result : {}",
        solve("day2/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day2::part_one::solve("input/part_one_input.txt"), 218);
    }
}
