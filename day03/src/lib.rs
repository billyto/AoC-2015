use anyhow::{Context, Result};
use std::{collections::HashSet, fs::read_to_string, i32};

// #[derive(Debug)]
// pub struct House {
//     north: Option<usize>,
//     south: Option<usize>,
//     west: Option<usize>,
//     east: Option<usize>,
// }

pub fn parse_input(input_path: String) -> Result<Vec<char>, anyhow::Error> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;
    let directions: Vec<char> = input_contents.chars().collect();

    Ok(directions)
}

pub fn solve_part1(directions: &Vec<char>) -> i32 {
    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let house_zero: (i32, i32) = (x, y);
    visited_houses.insert(house_zero);

    directions.iter().for_each(|&c| {
        match c {
            '^' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            'v' => y -= 1,
            _ => panic!("Unknown direction panic!"),
        };
        visited_houses.insert((x, y));
    });

    visited_houses.len() as i32
}

pub fn solve_part2(directions: &Vec<char>) -> i32 {
    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();

    let mut santa_x: i32 = 0;
    let mut santa_y: i32 = 0;

    let mut robo_x: i32 = 0;
    let mut robo_y: i32 = 0;

    let santa_zero: (i32, i32) = (santa_x, santa_y);
    visited_houses.insert(santa_zero);

    visited_houses.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        // Test case 1: ^>v<
        let input1 = "^>v<".chars().collect::<Vec<char>>();
        assert_eq!(solve_part1(&input1), 4);

        // Test case 2: ^v^v^v^v^v
        let input2 = "^v^v^v^v^v".chars().collect::<Vec<char>>();
        assert_eq!(solve_part1(&input2), 2);

        // Test case 3: >
        let input3 = ">".chars().collect::<Vec<char>>();
        assert_eq!(solve_part1(&input3), 2);
    }

    #[test]
    fn test_solve_part2() {
        // Test case 1: ^v
        let input1 = "^v".chars().collect::<Vec<char>>();
        assert_eq!(solve_part1(&input1), 3);

        // Test case 2: ^>v<
        let input2 = "^>v<".chars().collect::<Vec<char>>();
        assert_eq!(solve_part1(&input2), 3);

        // Test case 3: ^v^v^v^v^v
        let input3 = "^v^v^v^v^v".chars().collect::<Vec<char>>();
        assert_eq!(solve_part1(&input3), 11);
    }
}
