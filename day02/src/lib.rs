use anyhow::{Context, Result};

use itertools::Itertools;
use std::{fs::read_to_string, i32};

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
    let lenghts: Vec<i32> = dimensions_list
        .iter()
        .map(|d| {
            let bow_len: i32 = d.iter().fold(1, |acc, side| acc * side);
            //println!("bow len: {:?}", bow_len);

            let longest_side: &i32 = d
                .iter()
                // .enumerate()
                .max_by_key(|s| **s)
                //.map(|(index, _)| index)
                .unwrap();
            //println!("longest side: {:?}", longest_side);
            let ribbon_len1: i32 = d
                .iter()
                .filter(|side| side.ne(&longest_side))
                .map(|short_side| short_side * 2)
                .sum();
            //println!("ribbon len 1: {:?}", ribbon_len1);

            let mut sorted_lens: Vec<i32> = d.clone();
            sorted_lens.sort();
            let ribbon_len = (sorted_lens[0] * 2) + (sorted_lens[1] * 2);

            bow_len + ribbon_len
        })
        .collect::<Vec<i32>>();

    lenghts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![vec![2, 3, 4]]), 58);
        assert_eq!(solve_part1(&vec![vec![1, 1, 10]]), 43);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![vec![2, 3, 4]]), 34);
        assert_eq!(solve_part2(&vec![vec![1, 1, 10]]), 14);
        assert_eq!(solve_part2(&vec![vec![30, 22, 25]]), 16594)
    }
}
