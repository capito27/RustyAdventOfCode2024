use day7::part_one::solve;
fn main() {
    println!(
        "day7, part one result : {}",
        solve("day7/input/part_one_input.txt")
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_solution() {
        assert_eq!(
            day7::part_one::solve("input/part_one_input.txt"),
            1153997401072
        );
    }
}
