use anyhow::{Context, Result};
use std::fs::read_to_string;

pub fn parse_input(input_path: String) -> Result<Vec<String>, anyhow::Error> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;
    let strings: Vec<String> = input_contents.lines().map(String::from).collect();

    Ok(strings)
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    start: Coordinate,
    end: Coordinate,
}

impl Instruction {
    fn from_str(input: &str) -> Option<Instruction> {
        let segments: Vec<&str> = input.split_whitespace().collect();

        let (action, init_coordinates_position) = match segments[0] {
            "toggle" => (Action::Toggle, 1),
            "turn" => match segments[1] {
                "on" => (Action::TurnOn, 2),
                "off" => (Action::TurnOff, 2),
                _ => return None,
            },
            _ => return None,
        };

        let origin_coordinates: Vec<&str> =
            segments[init_coordinates_position].split(",").collect();
        let end_coordinates: Vec<&str> =
            segments[init_coordinates_position + 2].split(",").collect();

        Some(Instruction {
            action,
            start: Coordinate {
                x: origin_coordinates[0].parse().ok()?,
                y: origin_coordinates[1].parse().ok()?,
            },
            end: Coordinate {
                x: end_coordinates[0].parse().ok()?,
                y: end_coordinates[1].parse().ok()?,
            },
        })
    }
}

pub fn solve_part1(commands: &Vec<String>) -> u32 {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for command in commands {
        if let Some(instruction) = Instruction::from_str(command) {
            for i in instruction.start.y..=instruction.end.y {
                for j in instruction.start.x..=instruction.end.x {
                    match instruction.action {
                        Action::Toggle => {
                            if grid[i][j] == 0 {
                                grid[i][j] = 1
                            } else {
                                grid[i][j] = 0
                            }
                        }
                        Action::TurnOn => grid[i][j] = 1,
                        Action::TurnOff => grid[i][j] = 0,
                    }
                }
            }
        }
    }
    grid.iter().flatten().sum()
}

pub fn solve_part2(commands: &Vec<String>) -> u32 {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for command in commands {
        if let Some(instruction) = Instruction::from_str(command) {
            for i in instruction.start.y..=instruction.end.y {
                for j in instruction.start.x..=instruction.end.x {
                    match instruction.action {
                        Action::Toggle => grid[i][j] = grid[i][j] + 2,
                        Action::TurnOn => grid[i][j] = grid[i][j] + 1,
                        Action::TurnOff => {
                            if grid[i][j] != 0 {
                                grid[i][j] = grid[i][j] - 1
                            }
                        }
                    }
                }
            }
        }
    }
    grid.iter().flatten().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        // Test case 1: turn on all lights
        let input1 = "turn on 0,0 through 999,999";
        let result1 = solve_part1(&vec![input1.to_string()]);
        assert_eq!(result1, 1_000_000, "All lights should be on");

        // Test case 2: toggle first line
        let input2 = vec![
            "turn on 0,0 through 999,999".to_string(),
            "toggle 0,0 through 999,0".to_string(),
        ];
        let result2 = solve_part1(&input2);
        assert_eq!(
            result2, 999_000,
            "Should have all lights on except first row"
        );

        // Test case 3: turn off middle four lights
        let input3 = vec![
            "turn on 0,0 through 999,999".to_string(),
            "turn off 499,499 through 500,500".to_string(),
        ];
        let result3 = solve_part1(&input3);
        assert_eq!(
            result3, 999_996,
            "Should have all lights on except middle four"
        );

        // Test case 4: Combined operations
        let input4 = vec![
            "turn on 0,0 through 999,999".to_string(), // All on: 1,000,000
            "toggle 0,0 through 999,0".to_string(),    // First row toggled off: -1000
            "turn off 499,499 through 500,500".to_string(), // Middle four off: -4
        ];
        let result4 = solve_part1(&input4);
        assert_eq!(result4, 998_996, "Should process all operations correctly");
    }

    #[test]
    fn test_solve_part2() {
        // Test case 1: Single light increase
        let input1 = "turn on 0,0 through 0,0";
        let result1 = solve_part2(&vec![input1.to_string()]);
        assert_eq!(result1, 1, "Single light should increase by 1");

        // Test case 2: Toggle all lights
        let input2 = "toggle 0,0 through 999,999";
        let result2 = solve_part2(&vec![input2.to_string()]);
        assert_eq!(result2, 2_000_000, "Toggle should increase all lights by 2");

        // Test case 3: Combined operations
        let input3 = vec![
            "turn on 0,0 through 0,0".to_string(),    // +1 for one light
            "toggle 0,0 through 999,999".to_string(), // +2 for all lights
        ];
        let result3 = solve_part2(&input3);
        assert_eq!(
            result3, 2_000_001,
            "Should handle combined operations correctly"
        );
    }
}
