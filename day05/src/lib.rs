use anyhow::{Context, Result};
use regex::Regex;
use std::{fs::read_to_string, i32};

pub fn parse_input(input_path: String) -> Result<Vec<String>, anyhow::Error> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;
    let strings: Vec<String> = input_contents.lines().map(String::from).collect();

    Ok(strings)
}

fn has_doubles(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b)
}

pub fn solve_part1(strings: &Vec<String>) -> usize {
    let vowels_trio = Regex::new("^(.*[aeiou]){3,}.*$").unwrap();
    //let double_pairs = Regex::new(r"(.)\1").unwrap(); // \1 not supported
    let banned_pairs = Regex::new(r"ab|cd|pq|xy").unwrap();

    strings
        .iter()
        .filter(|&s| vowels_trio.is_match(s) && has_doubles(s) && !banned_pairs.is_match(s))
        .count()
}

pub fn solve_part2(strings: &Vec<String>) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "ugknbfddgicrmopn".to_string(),
            "aaa".to_string(),
            "jchzalrnumimnmhp".to_string(),
            "haegwjzuvuyypxyu".to_string(),
            "dvszwmarrgswjxmb".to_string(),
        ];

        assert_eq!(solve_part1(&input), 2);
    }

    // You can also test individual strings if you want to be more specific
    #[test]
    fn test_individual_strings() {
        assert_eq!(solve_part1(&vec!["ugknbfddgicrmopn".to_string()]), 1);
        assert_eq!(solve_part1(&vec!["aaa".to_string()]), 1);
        assert_eq!(solve_part1(&vec!["jchzalrnumimnmhp".to_string()]), 0);
        assert_eq!(solve_part1(&vec!["haegwjzuvuyypxyu".to_string()]), 0);
        assert_eq!(solve_part1(&vec!["dvszwmarrgswjxmb".to_string()]), 0);
    }
}
