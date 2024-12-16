use day8::part_one::solve;
fn main() {
    println!(
        "day8, part one result : {}",
        solve("day8/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(day8::part_one::solve("input/part_one_input.txt"), 313);
    }
}
