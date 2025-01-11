use anyhow::{Context, Result};

use itertools::Itertools;
use std::{fs::read_to_string, i32, ops::Div};

pub fn parse_input(input_path: String) -> Result<Vec<Vec<i32>>, anyhow::Error> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;

    let dimensions: Vec<Vec<i32>> = input_contents
        .lines()
        .map(|line| -> Vec<i32> { line.split('x').map(|s| s.parse().unwrap()).collect() })
        .collect();

    Ok(dimensions)
}

pub fn solve_part1(dimensions_list: &Vec<Vec<i32>>) -> i32 {
    let areas: Vec<i32> = dimensions_list
        .iter()
        .map(|d| {
            //println!("here is D:{:?}", d);
            let sides: Vec<i32> = d.iter().tuple_combinations().map(|(a, b)| a * b).collect();
            let min_side: i32 = sides.iter().min().unwrap().clone();
            //println!("{:?}", sides);
            let area: i32 = sides.iter().map(|s| s * 2).sum();
            area + min_side //slack
        })
        .collect::<Vec<i32>>();

    areas.iter().sum()
}

pub fn solve_part2(dimensions_list: &Vec<Vec<i32>>) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![vec![2, 3, 4]]), 58);
        assert_eq!(solve_part1(&vec![vec![1, 1, 10]]), 43);
    }
}
