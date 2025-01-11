use anyhow::Result;
use day02::{parse_input, solve_part1};

fn main() -> Result<()> {
    // Read the input file that matches the Cargo Package name
    let input_path = format!("../inputs/{}.txt", env!("CARGO_PKG_NAME"));

    let dimensions = parse_input(input_path)?;

    println!("Part 1: {}", solve_part1(&dimensions));
    // println!("Part 2: {}", solve_part2(&dimensions));
    Ok(())
}
