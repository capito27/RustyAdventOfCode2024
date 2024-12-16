use day10::part_one::solve;
fn main() {
    println!(
        "day10, part one result : {}",
        solve("day10/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day10::part_one::solve("input/part_one_input.txt"), 535);
    }
}
