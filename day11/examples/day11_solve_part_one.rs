use day11::part_one::solve;
fn main() {
    println!(
        "day11, part one result : {}",
        solve("day11/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day11::part_one::solve("input/part_one_input.txt"), 182081);
    }
}
