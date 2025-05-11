use anyhow::{Context, Result};
use std::fs::read_to_string;

pub fn parse_input(input_path: String) -> Result<Vec<String>> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;
    let strings: Vec<String> = input_contents.lines().map(String::from).collect();

    Ok(strings)
}

pub fn solve_part1(strings: &[String]) -> usize {
    // Calculate difference between code representation and in-memory size
    strings
        .iter()
        .map(|s| code_length(s) - memory_length(s))
        .sum()
}

// Helper functions
fn code_length(s: &str) -> usize {
    // Return the original code lengthi
    s.len()
}

fn memory_length(s: &str) -> usize {
    let mut chars = s.chars().peekable();
    let mut count = 0;

    // Skip the opening quote
    if chars.next() == Some('"') {
        // Process the string content
        while let Some(c) = chars.next() {
            match c {
                '"' if chars.peek().is_none() => {
                    // Closing quote, don't count it
                    break;
                }
                '\\' => {
                    // Handle escape sequences
                    if let Some(next) = chars.next() {
                        match next {
                            '\\' | '"' => {
                                // Escaped backslash or quote counts as 1 character
                                count += 1;
                            }
                            'x' => {
                                // Hex escape sequence \x##
                                // Skip the two hex digits
                                chars.next();
                                chars.next();
                                count += 1;
                            }
                            _ => {
                                // Invalid escape sequence, but we'll count it anyway
                                count += 1;
                            }
                        }
                    }
                }
                _ => {
                    // Regular character
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn solve_part2(strings: &[String]) -> usize {
    // Calculate difference between code representation and in-memory size
    strings
        .iter()
        .map(|s| encoded_length(s) - code_length(s))
        .sum()
}

fn encoded_length(s: &str) -> usize {
    // encode the code representation as a new string and get the number of charaters of the resulting string
    // Start with 2 for the enclosing quotes
    let mut count = 2;

    // Process each character
    for c in s.chars() {
        match c {
            '"' | '\\' => {
                // Each quote or backslash needs to be escaped with a backslash
                count += 2;
            }
            _ => {
                // All other characters are copied as-is
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_length() {
        // Test cases from the problem description
        assert_eq!(memory_length(r#""""#), 0); // Empty string
        assert_eq!(memory_length(r#""abc""#), 3); // Simple string
        assert_eq!(memory_length(r#""aaa\"aaa""#), 7); // String with escaped quote
        assert_eq!(memory_length(r#""\x27""#), 1); // String with hex escape

        // Additional test cases
        assert_eq!(memory_length(r#""\\""#), 1); // Single escaped backslash
        assert_eq!(memory_length(r#""\\\"""#), 2); // Escaped backslash followed by escaped quote
        assert_eq!(memory_length(r#""Hello\nWorld""#), 11); // Invalid escape sequence (counts as 'n')
        assert_eq!(memory_length(r#""Hello\x20World""#), 11); // Space character as hex
        assert_eq!(memory_length(r#""\\x27""#), 4); // Not a hex escape (escaped \ followed by x27)
    }

    #[test]
    fn test_encoded_length() {
        // Test cases
        assert_eq!(encoded_length(r#""""#), 6); // "" -> \"\"
        assert_eq!(encoded_length(r#""abc""#), 9); // "abc" -> \"abc\"
        assert_eq!(encoded_length(r#""aaa\"aaa""#), 16); // "aaa\"aaa" -> \"aaa\\\"aaa\"
        assert_eq!(encoded_length(r#""\x27""#), 11); // "\x27" -> \"\\x27\"

        // Additional test cases
        assert_eq!(encoded_length(r#""\\""#), 10); // "\\" -> \"\\\\\"
        assert_eq!(encoded_length(r#""\\\"""#), 14); // "\\\"" -> \"\\\\\\\"\"
                                                     // assert_eq!(encoded_length(r#""Hello\nWorld""#), 17); // "Hello\nWorld" -> \"Hello\\nWorld\"
                                                     // assert_eq!(encoded_length(r#""\\x27""#), 14); // "\\x27" -> \"\\\\x27\"
    }
}
