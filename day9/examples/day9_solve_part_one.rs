use day9::part_one::solve;
fn main() {
    println!(
        "day9, part one result : {}",
        solve("day9/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(
            day9::part_one::solve("input/part_one_input.txt"),
            6344673854800
        );
    }
}
