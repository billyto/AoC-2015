use anyhow::{Context, Result};
use md5::{Digest, Md5};
use std::{fs::read_to_string, i32};

pub fn parse_input(input_path: String) -> Result<String, anyhow::Error> {
    let secret_key: String = read_to_string(input_path).context("Could not read input file")?;

    Ok(secret_key)
}

pub fn solve_part1(secret_key: &String) -> i32 {
    for n in 0..100_000_000 {
        let candidate = format!("{}{}", secret_key, n);
        let hash = format!("{:x}", Md5::digest(candidate.as_bytes()));
        println!("{}", hash);

        if hash.starts_with("00000") {
            return n;
        }
    }
    0
}

pub fn solve_part2(secret_key: &String) -> i32 {
    for n in 0..i32::MAX {
        let candidate = format!("{}{}", secret_key, n);
        let hash = format!("{:x}", Md5::digest(candidate.as_bytes()));
        println!("{}", hash);

        if hash.starts_with("000000") {
            return n;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&"abcdef".to_string()), 609043);
        assert_eq!(solve_part1(&"pqrstuv".to_string()), 1048970);
    }
}
