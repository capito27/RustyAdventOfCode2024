use day3::part_one::solve;
fn main() {
    println!(
        "day3, part one result : {}",
        solve("day3/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day3::part_one::solve("input/part_one_input.txt"), 159892596);
    }
}
