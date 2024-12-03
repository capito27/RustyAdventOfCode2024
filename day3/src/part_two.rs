use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

pub fn solve<P>(input_file: P) -> u32
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap().replace("\n", ""); // Remove newlines
                                                                       // This regex will only capture all sections that either start with do or are the beginning, and end with don't or the end, in a lazy fashion
    let pre_process_regex = Regex::new(r"(^|do\(\)).+?(don't\(\)|$)").unwrap();
    let input = pre_process_regex
        .find_iter(&input)
        .map(|m| m.as_str())
        .collect::<String>();
    Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
        .unwrap()
        .find_iter(&input)
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
