use std::fs::read_to_string;
use anyhow::{Context, Result};
use regex::Regex;


pub fn parse_input(input_path: String) -> anyhow::Result<String> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;

    Ok(input_contents)
}

fn extract_numbers(input: &str) -> Result<Vec<i32>> {
    let re = Regex::new(r"-?\d+") //positive and negative integers
        .context("Failed to compile regex")?;

    let numbers = re
        .find_iter(input)
        .map(|mat| {
            mat.as_str()
                .parse::<i32>()
                .with_context(|| format!("Failed to parse number: {}", mat.as_str()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(numbers)
}

fn find_object_boundaries(input: &str, red_pos: usize) -> Option<(usize, usize)> {
    let chars: Vec<char> = input.chars().collect();

    // Find the innermost container that directly contains "red"
    // We need to find the closest opening brace/bracket that hasn't been closed yet
    let mut start_pos = None;
    let mut container_type = None;
    let mut depth = 0;

    // Go backwards from red_pos to find the immediate container
    for i in (0..red_pos).rev() {
        match chars[i] {
            '}' | ']' => depth += 1,
            '{' => {
                if depth == 0 {
                    // This is the immediate containing object
                    start_pos = Some(i);
                    container_type = Some('o');
                    break;
                } else {
                    depth -= 1;
                }
            }
            '[' => {
                if depth == 0 {
                    // This is the immediate containing array
                    start_pos = Some(i);
                    container_type = Some('a');
                    break;
                } else {
                    depth -= 1;
                }
            }
            _ => {}
        }
    }

    let start = start_pos?;
    let container = container_type?;

    // For AoC 2015 Day 12: only ignore objects, not arrays
    if container == 'a' {
        return None;
    }

    // Find the matching closing brace for this specific object
    let mut end_pos = None;
    depth = 0;

    // Start from the opening brace and find its matching closing brace
    for i in (start + 1)..chars.len() {
        match chars[i] {
            '{' | '[' => depth += 1,
            '}' => {
                if depth == 0 {
                    end_pos = Some(i);
                    break;
                } else {
                    depth -= 1;
                }
            }
            ']' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {}
        }
    }

    let end = end_pos?;
    Some((start, end))
}


fn remove_red_objects(input: &str) -> String {
    let mut result = input.to_string();

    while let Some(red_pos) = result.find("\"red\"") {
        // Find the object boundaries that contain this "red"
        if let Some((start, end)) = find_object_boundaries(&result, red_pos) {
            // Extract the object content
            let object_content = &result[start..=end];

            // Replace numbers in this object with zeros AND replace "red" with "removed"
            let re = regex::Regex::new(r"-?\d+").unwrap();
            let mut zeroed_content = re.replace_all(object_content, "0").to_string();
            zeroed_content = zeroed_content.replace("\"red\"", "\"removed\"");

            // Replace the object in the result string
            result.replace_range(start..end+1, &zeroed_content);
        } else {
            // If we can't find boundaries (e.g., red is in an array), just remove this "red" to avoid infinite loop
            result = result.replacen("\"red\"", "\"removed\"", 1);
        }
    }

    result
}

pub fn solve_part1(string: &String) -> i32 {

    match extract_numbers(string) {
        Ok(numbers) => numbers.iter().sum(),
        Err(e) => {
            eprintln!("Error at numbers extraction: {}", e);
            0
        }
    }
}

pub fn solve_part2(string: &String) -> i32 {

    let cleaned_input = remove_red_objects(string);
    match extract_numbers(&cleaned_input) {
        Ok(numbers) => numbers.iter().sum(),
        Err(e) => {
            eprintln!("Something went wrong extracting the numbers: {}", e);
            0
        }
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let input = r#"{"a":{"b":4},"c":-1,"d":{"e":{"f":6}}}"#;
        let numbers = extract_numbers(input).unwrap();
        assert_eq!(numbers, vec![4, -1, 6]);
    }

    #[test]
    fn test_extract_numbers_array() {
        let input = "[1,2,3]";
        let numbers = extract_numbers(input).unwrap();
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_extract_numbers_mixed() {
        let input = r#"[1,{"c":"red","b":2},3]"#;
        let numbers = extract_numbers(input).unwrap();
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_red_objects() {
        let input = r#"{"a":1,"b":"red","c":2}"#;
        let cleaned = remove_red_objects(input);
        let numbers = extract_numbers(&cleaned).unwrap();
        assert_eq!(numbers.iter().sum::<i32>(), 0); // All numbers should be zeroed
    }

    #[test]
    fn test_remove_red_in_array() {
        let input = r#"[1,2,"red",3]"#;
        let cleaned = remove_red_objects(input);
        let numbers = extract_numbers(&cleaned).unwrap();
        assert_eq!(numbers.iter().sum::<i32>(), 6); // Array with red should NOT be ignored
    }

    #[test]
    fn test_remove_red_nested() {
        let input = r#"{"a":{"b":"red","c":5},"d":6}"#;
        let cleaned = remove_red_objects(input);
        let numbers = extract_numbers(&cleaned).unwrap();
        assert_eq!(numbers.iter().sum::<i32>(), 6); // Only 6 should remain
    }

    #[test]
    fn test_no_red_objects() {
        let input = r#"{"a":1,"b":2,"c":3}"#;
        let cleaned = remove_red_objects(input);
        let numbers = extract_numbers(&cleaned).unwrap();
        assert_eq!(numbers.iter().sum::<i32>(), 6); // All numbers should remain
    }
}