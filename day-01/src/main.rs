use anyhow::{Context, Result};
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use std::{fs::read_to_string, i32};

fn solve_part1(directions: &Vec<i32>) -> i32 {
    directions.iter().sum()
}

fn solve_part2(directions: &Vec<i32>) -> usize {
    let (_, last_index) = directions
        .iter()
        .enumerate()
        .fold_while((0, 0), |(acc, _), (idx, &x)| {
            if acc == -1 {
                Done((acc, idx))
            } else {
                Continue((acc + x, idx))
            }
        })
        .into_inner();
    last_index
}

fn main() -> Result<()> {
    // Read input from a file in the current day's directory
    let input_path = "input.txt";
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;

    let directions: Vec<i32> = input_contents
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect();

    println!("Part 1: {}", solve_part1(&directions));
    println!("Part 2: {}", solve_part2(&directions));
    Ok(())
}
