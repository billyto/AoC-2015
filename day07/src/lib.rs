use anyhow::{anyhow, Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

// define types for the circuit
#[derive(Debug, Clone)]
pub enum Signal {
    Value(u16),
    Wire(String),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Assign(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    LShift(Signal, u16),
    RShift(Signal, u16),
    Not(Signal),
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub target: String,
}

pub fn parse_input(input_path: String) -> Result<Vec<String>, anyhow::Error> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;
    let strings: Vec<String> = input_contents.lines().map(String::from).collect();

    Ok(strings)
}

// Parse a signal
fn parse_signal(signal_str: &str) -> Result<Signal, anyhow::Error> {
    if let Ok(value) = signal_str.parse::<u16>() {
        Ok(Signal::Value(value))
    } else {
        Ok(Signal::Wire(signal_str.to_string()))
    }
}

// Parse an instruction
pub fn parse_instruction(instruction_str: &str) -> Result<Instruction, anyhow::Error> {
    let parts: Vec<&str> = instruction_str.split(" -> ").collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid instruction format"));
    }

    let target = parts[1].to_string();
    let operation_parts: Vec<&str> = parts[0].split_whitespace().collect();

    let operation = match operation_parts.len() {
        1 => Operation::Assign(parse_signal(operation_parts[0])?),
        2 if operation_parts[0] == "NOT" => Operation::Not(parse_signal(operation_parts[1])?),
        3 => match operation_parts[1] {
            "AND" => Operation::And(
                parse_signal(operation_parts[0])?,
                parse_signal(operation_parts[2])?,
            ),
            "OR" => Operation::Or(
                parse_signal(operation_parts[0])?,
                parse_signal(operation_parts[2])?,
            ),
            "LSHIFT" => {
                let value = operation_parts[2]
                    .parse::<u16>()
                    .with_context(|| format!("Invalid shift value: {}", operation_parts[2]))?;
                Operation::LShift(parse_signal(operation_parts[0])?, value)
            }
            "RSHIFT" => {
                let value = operation_parts[2]
                    .parse::<u16>()
                    .with_context(|| format!("Invalid shift value: {}", operation_parts[2]))?;
                Operation::RShift(parse_signal(operation_parts[0])?, value)
            }
            _ => return Err(anyhow::anyhow!("Unknown operation: {}", operation_parts[1])),
        },
        _ => return Err(anyhow::anyhow!("Invalid operation format: {}", parts[0])),
    };

    Ok(Instruction { target, operation })
}

// Evaluate a wire's value with stack-based DFS
fn evaluate_wire(
    wire: &str,
    instructions_map: &HashMap<String, Instruction>,
    memo: &mut HashMap<String, u16>,
) -> Result<u16> {
    // Return memoized value if available
    if let Some(&value) = memo.get(wire) {
        return Ok(value);
    }

    let mut stack = VecDeque::new();
    let mut in_progress = HashSet::new();
    let mut ready = HashSet::new();

    // Add the starting wire to our stack
    stack.push_back((wire.to_string(), false));

    while let Some((current_wire, is_processing)) = stack.pop_back() {
        if is_processing {
            // We're processing this wire now, which means all dependencies have been evaluated
            if ready.contains(&current_wire) {
                continue; // Already processed
            }

            // Get the instruction for this wire
            let instruction = instructions_map
                .get(&current_wire)
                .ok_or_else(|| anyhow!("No instruction found for wire: {}", current_wire))?;

            // Evaluate the operation
            let value = match &instruction.operation {
                Operation::Assign(signal) => evaluate_signal(signal, memo)?,
                Operation::And(left, right) => {
                    evaluate_signal(left, memo)? & evaluate_signal(right, memo)?
                }
                Operation::Or(left, right) => {
                    evaluate_signal(left, memo)? | evaluate_signal(right, memo)?
                }
                Operation::LShift(signal, amount) => evaluate_signal(signal, memo)? << amount,
                Operation::RShift(signal, amount) => evaluate_signal(signal, memo)? >> amount,
                Operation::Not(signal) => !evaluate_signal(signal, memo)?,
            };

            // Memoize the result
            memo.insert(current_wire.clone(), value);
            ready.insert(current_wire.clone());
            in_progress.remove(&current_wire);
        } else {
            // We're visiting this wire for the first time
            if ready.contains(&current_wire) {
                continue; // Already processed
            }

            if in_progress.contains(&current_wire) {
                return Err(anyhow!(
                    "Cycle detected in circuit at wire {}",
                    current_wire
                ));
            }

            in_progress.insert(current_wire.clone());

            // Schedule this wire to be processed after dependencies
            stack.push_back((current_wire.clone(), true));

            // Get dependencies and add them to the stack
            if let Some(instruction) = instructions_map.get(&current_wire) {
                match &instruction.operation {
                    Operation::Assign(signal) => {
                        if let Signal::Wire(dep_wire) = signal {
                            if !ready.contains(dep_wire) {
                                stack.push_back((dep_wire.clone(), false));
                            }
                        }
                    }
                    Operation::And(left, right) | Operation::Or(left, right) => {
                        if let Signal::Wire(dep_wire) = left {
                            if !ready.contains(dep_wire) {
                                stack.push_back((dep_wire.clone(), false));
                            }
                        }
                        if let Signal::Wire(dep_wire) = right {
                            if !ready.contains(dep_wire) {
                                stack.push_back((dep_wire.clone(), false));
                            }
                        }
                    }
                    Operation::LShift(signal, _) | Operation::RShift(signal, _) => {
                        if let Signal::Wire(dep_wire) = signal {
                            if !ready.contains(dep_wire) {
                                stack.push_back((dep_wire.clone(), false));
                            }
                        }
                    }
                    Operation::Not(signal) => {
                        if let Signal::Wire(dep_wire) = signal {
                            if !ready.contains(dep_wire) {
                                stack.push_back((dep_wire.clone(), false));
                            }
                        }
                    }
                }
            } else {
                return Err(anyhow!("No instruction found for wire: {}", current_wire));
            }
        }
    }

    // Return the memoized value
    memo.get(wire)
        .copied()
        .ok_or_else(|| anyhow!("Wire {} value not calculated", wire))
}

// Helper function to evaluate signals (either direct values or wire references)
fn evaluate_signal(signal: &Signal, memo: &HashMap<String, u16>) -> Result<u16> {
    match signal {
        Signal::Value(val) => Ok(*val),
        Signal::Wire(wire) => memo
            .get(wire)
            .copied()
            .ok_or_else(|| anyhow!("Wire {} value not found in memo", wire)),
    }
}

pub fn solve_part1(lines: &Vec<String>) -> u16 {
    // Parse instructions
    let mut instructions = Vec::new();
    for line in lines {
        match parse_instruction(line) {
            Ok(inst) => instructions.push(inst),
            Err(e) => {
                eprintln!("Error parsing instruction '{}': {}", line, e);
                continue;
            }
        }
    }

    // Build a map of wire -> instruction
    let instructions_map: HashMap<String, Instruction> = instructions
        .iter()
        .map(|inst| (inst.target.clone(), inst.clone()))
        .collect();

    // Evaluate and return the value of wire 'a'
    let mut memo = HashMap::new();
    match evaluate_wire("a", &instructions_map, &mut memo) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error evaluating wire 'a': {}", e);
            0
        }
    }
}

pub fn solve_part2(lines: &Vec<String>) -> u16 {
    // Parse instructions
    let mut instructions = Vec::new();
    for line in lines {
        match parse_instruction(line) {
            Ok(inst) => instructions.push(inst),
            Err(e) => {
                eprintln!("Error parsing instruction '{}': {}", line, e);
                continue;
            }
        }
    }

    // First, calculate the value of wire 'a'
    let a_value = solve_part1(lines);

    // Now, override wire 'b' with the value of 'a'
    for inst in &mut instructions {
        if inst.target == "b" {
            inst.operation = Operation::Assign(Signal::Value(a_value));
        }
    }

    // Build a map of wire -> instruction with the modified instructions
    let instructions_map: HashMap<String, Instruction> = instructions
        .iter()
        .map(|inst| (inst.target.clone(), inst.clone()))
        .collect();

    // Evaluate and return the new value of wire 'a'
    let mut memo = HashMap::new();
    match evaluate_wire("a", &instructions_map, &mut memo) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error evaluating wire 'a' in part 2: {}", e);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() -> Result<()> {
        // Test direct assignment
        let inst = parse_instruction("123 -> x")?;
        assert_eq!(inst.target, "x");
        if let Operation::Assign(Signal::Value(val)) = inst.operation {
            assert_eq!(val, 123);
        } else {
            panic!("Expected Assign operation with Value signal");
        }

        // Test wire assignment
        let inst = parse_instruction("y -> x")?;
        assert_eq!(inst.target, "x");
        if let Operation::Assign(Signal::Wire(wire)) = inst.operation {
            assert_eq!(wire, "y");
        } else {
            panic!("Expected Assign operation with Wire signal");
        }

        // Test AND operation
        let inst = parse_instruction("x AND y -> z")?;
        assert_eq!(inst.target, "z");
        if let Operation::And(Signal::Wire(left), Signal::Wire(right)) = &inst.operation {
            assert_eq!(left, "x");
            assert_eq!(right, "y");
        } else {
            panic!("Expected AND operation");
        }

        // Test OR operation
        let inst = parse_instruction("x OR y -> z")?;
        assert_eq!(inst.target, "z");
        if let Operation::Or(Signal::Wire(left), Signal::Wire(right)) = &inst.operation {
            assert_eq!(left, "x");
            assert_eq!(right, "y");
        } else {
            panic!("Expected OR operation");
        }

        // Test AND with value and wire
        let inst = parse_instruction("1 AND y -> z")?;
        assert_eq!(inst.target, "z");
        if let Operation::And(Signal::Value(val), Signal::Wire(right)) = &inst.operation {
            assert_eq!(*val, 1);
            assert_eq!(right, "y");
        } else {
            panic!("Expected AND operation with Value and Wire");
        }

        // Test LSHIFT operation
        let inst = parse_instruction("x LSHIFT 2 -> z")?;
        assert_eq!(inst.target, "z");
        if let Operation::LShift(Signal::Wire(wire), shift) = &inst.operation {
            assert_eq!(wire, "x");
            assert_eq!(*shift, 2);
        } else {
            panic!("Expected LSHIFT operation");
        }

        // Test RSHIFT operation
        let inst = parse_instruction("x RSHIFT 2 -> z")?;
        assert_eq!(inst.target, "z");
        if let Operation::RShift(Signal::Wire(wire), shift) = &inst.operation {
            assert_eq!(wire, "x");
            assert_eq!(*shift, 2);
        } else {
            panic!("Expected RSHIFT operation");
        }

        // Test NOT operation
        let inst = parse_instruction("NOT x -> z")?;
        assert_eq!(inst.target, "z");
        if let Operation::Not(Signal::Wire(wire)) = &inst.operation {
            assert_eq!(wire, "x");
        } else {
            panic!("Expected NOT operation");
        }

        // Test invalid formats
        assert!(parse_instruction("invalid").is_err());
        assert!(parse_instruction("x INVALID y -> z").is_err());
        assert!(parse_instruction("x LSHIFT y -> z").is_err()); // Shift amount must be a number

        Ok(())
    }
}
