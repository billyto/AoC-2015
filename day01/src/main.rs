use anyhow::Result;
use day01::{parse_input, solve_part1, solve_part2};

fn main() -> Result<()> {
    // Read the input file that matches the Cargo Package name
    let input_path = format!("../inputs/{}.txt", env!("CARGO_PKG_NAME"));
    let directions = parse_input(input_path)?;

    println!("Part 1: {}", solve_part1(&directions));
    println!("Part 2: {}", solve_part2(&directions));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&vec![1, -1]), 0);
        assert_eq!(solve_part1(&vec![1, 1, 1]), 3);
        assert_eq!(solve_part1(&vec![-1, -1, -1]), -3);
        assert_eq!(solve_part1(&vec![1, 1, -1, -1]), 0);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&vec![-1]), 1);
        assert_eq!(solve_part2(&vec![1, 1, -1, -1, -1]), 5);
        assert_eq!(solve_part2(&vec![-1, -1]), 1);
        assert_eq!(solve_part2(&vec![-1, 1]), 1);
    }
}
