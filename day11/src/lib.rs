use std::fs::read_to_string;
use anyhow::Context;


pub fn parse_input(input_path: String) -> anyhow::Result<String> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;

    Ok(input_contents)
}

fn increment_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    let mut i = chars.len() - 1;

    loop {
        if chars[i] == 'z' {
            chars[i] = 'a';
            if i == 0 {
                //Handle overflow 
                chars.insert(0, 'a');
                break;
            }
            i -= 1;
        } else {
            chars[i] = (chars[i] as u8 + 1) as char;
            break;
        }
    }

    chars.iter().collect()
}


fn has_straight(password: &str) -> bool {
    password.as_bytes()
        .windows(3)
        .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
}

fn has_no_forbidden_chars(password: &str) -> bool {
    let forbidden = ['i', 'o', 'l'];
    !password.chars().any(|c| forbidden.contains(&c))
}

fn has_two_pairs(password: &str) -> bool {
    let mut found_pairs = std::collections::HashSet::new();
    let chars: Vec<char> = password.chars().collect();

    let mut i = 0;
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            found_pairs.insert(chars[i]);
            i += 2; // Skip to avoid overlapping pairs
        } else {
            i += 1;
        }
    }

    found_pairs.len() >= 2
}

fn is_a_good_password(password: &str) -> bool {
    has_straight(password) && 
        has_no_forbidden_chars(password) && 
        has_two_pairs(password)
}

pub fn solve_part1(current_password: &String) -> String {

    let mut password = current_password.clone();
    
    loop {
        password = increment_password(&password);
        if is_a_good_password(&password) {
            return password;
        }
    }
    
    
}


pub fn solve_part2(string: &String) -> String { 
    
    let first_valid_password = solve_part1(&string);
    solve_part1(&first_valid_password)
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_password() {
        assert_eq!(increment_password("xx"), "xy");
        assert_eq!(increment_password("xy"), "xz");
        assert_eq!(increment_password("xz"), "ya");
        assert_eq!(increment_password("ya"), "yb");
    }

    #[test]
    fn test_has_straight() {
        assert!(has_straight("hijklmmn"));
        assert!(!has_straight("abbceffg"));
        assert!(!has_straight("abbcegjk"));
    }

    #[test]
    fn test_has_forbidden_letters() {
        assert!(!has_no_forbidden_chars("hijklmmn"));
        assert!(has_no_forbidden_chars("abbceffg"));
    }

    #[test]
    fn test_has_two_pairs() {
        assert!(has_two_pairs("abbceffg"));
        assert!(!has_two_pairs("abbcegjk"));
        assert!(has_two_pairs("abcdffaa"));
    }

    #[test]
    fn test_is_valid_password() {
        assert!(!is_a_good_password("hijklmmn")); // has forbidden letter
        assert!(!is_a_good_password("abbceffg")); // no straight
        assert!(!is_a_good_password("abbcegjk")); // no pairs
        assert!(is_a_good_password("abcdffaa")); // valid
    }
}