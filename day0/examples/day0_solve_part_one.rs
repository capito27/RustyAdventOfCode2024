use day0::part_one::solve;
fn main() {
    println!(
        "day0, part one result : {}",
        solve("day0/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day0::part_one::solve("input/part_one_input.txt"), 0);
    }
}
