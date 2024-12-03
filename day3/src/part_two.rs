use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap().replace("\n", "");
    let pre_process_regex = Regex::new(r"(^|do\(\)).+?(don't\(\)|$)").unwrap();
    let solving_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    solving_regex
        .find_iter(
            pre_process_regex
                .find_iter(&input)
                .map(|m| m.as_str())
                .collect::<String>()
                .as_str(),
        )
        .map(|mul| {
            mul.as_str()
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(",")
                .unwrap()
        })
        .map(|(l, r)| l.parse::<u32>().unwrap() * r.parse::<u32>().unwrap())
        .sum::<u32>()
}
