use day6::part_one::solve;
fn main() {
    println!(
        "day6, part one result : {}",
        solve("day6/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day6::part_one::solve("input/part_one_input.txt"), 5516);
    }
}
