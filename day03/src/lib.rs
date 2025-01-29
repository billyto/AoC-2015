use anyhow::{Context, Result};
use std::{collections::HashSet, fs::read_to_string, i32};

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
            _ => panic!("Unknown direction panic: {}", c),
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

    for (index, c) in directions.iter().enumerate() {
        let is_santa: bool = index % 2 == 0;

        match c {
            '^' => {
                if is_santa {
                    santa_y += 1
                } else {
                    robo_y += 1
                }
            }
            '<' => {
                if is_santa {
                    santa_x -= 1
                } else {
                    robo_x -= 1
                }
            }
            '>' => {
                if is_santa {
                    santa_x += 1
                } else {
                    robo_x += 1
                }
            }
            'v' => {
                if is_santa {
                    santa_y -= 1
                } else {
                    robo_y -= 1
                }
            }
            _ => panic!("Unknown direction panic!"),
        };

        let house: (i32, i32) = if is_santa {
            (santa_x, santa_y)
        } else {
            (robo_x, robo_y)
        };
        // println!(
        //     "{} going to {:?}",
        //     if is_santa { "Santa" } else { "Robo" },
        //     house
        // );
        visited_houses.insert(house);
    }

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
        assert_eq!(solve_part2(&input1), 3);

        // Test case 2: ^>v<
        let input2 = "^>v<".chars().collect::<Vec<char>>();
        assert_eq!(solve_part2(&input2), 3);

        // Test case 3: ^v^v^v^v^v
        let input3 = "^v^v^v^v^v".chars().collect::<Vec<char>>();
        assert_eq!(solve_part2(&input3), 11);
    }
}
