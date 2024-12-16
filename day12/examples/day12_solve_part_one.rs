use day12::part_one::solve;
fn main() {
    println!(
        "day12, part one result : {}",
        solve("day12/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day12::part_one::solve("input/part_one_input.txt"), 0);
    }
}
