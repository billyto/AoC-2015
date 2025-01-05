use anyhow::{Context, Result};
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use std::{fs::read_to_string, i32};

pub fn parse_input(input_path: String) -> Result<Vec<i32>, anyhow::Error> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;
    let directions: Vec<i32> = input_contents
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect();
    Ok(directions)
}

pub fn solve_part1(directions: &Vec<i32>) -> i32 {
    directions.iter().sum()
}

pub fn solve_part2(directions: &Vec<i32>) -> usize {
    let (_, last_index) = directions
        .iter()
        .enumerate()
        .fold_while((0, 0), |(acc, _), (idx, &x)| {
            if acc + x == -1 {
                Done((acc, idx))
            } else {
                Continue((acc + x, idx))
            }
        })
        .into_inner();
    last_index + 1 // It's a 1-based floor counting
}
