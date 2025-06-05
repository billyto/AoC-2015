use std::fs::read_to_string;
use anyhow::Context;

pub fn parse_input(input_path: String) -> anyhow::Result<String> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;

    Ok(input_contents)
}


fn has_increasing_straight(s: &str) -> bool {
    s.as_bytes()
        .windows(3)
        .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
}

fn has_no_forbidden_chars(s: &str) -> bool {
    let forbidden = ['i', 'o', 'l'];
    !s.chars().any(|c| forbidden.contains(&c))
}

fn has_two_different_pairs(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    if chars.len() < 2 {
        return false;
    }
    
    let mut found_pairs = std::collections::HashSet::new();
    let mut skip_next = false;

    for i in 0..chars.len() - 1 {
        if skip_next {
            skip_next = false;
            continue;
        }

        if chars[i] == chars[i + 1] {
            found_pairs.insert(chars[i]);
            skip_next = true; // For non-overlapping
        }
    }

    found_pairs.len() >= 2
}



pub fn solve_part1(string: &String) -> usize {


    let letters: Vec<char> = ('a'..='z')
        .filter(|&c| c != 'i' && c != 'o' && c != 'l')
        .collect();
    43
    
}


pub fn solve_part2(string: &String) -> usize { 43 }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_no_forbidden_chars() {
        assert!(has_no_forbidden_chars(""));          // empty string
        assert!(has_no_forbidden_chars("abcdefg"));   // no forbidden 

        assert!(!has_no_forbidden_chars("hello"));    // contains 'l' and 'o'
        assert!(!has_no_forbidden_chars("password")); // contains 'o'
        assert!(!has_no_forbidden_chars("inside"));   // contains 'i'
        assert!(!has_no_forbidden_chars("oil"));      // contains 'o', 'i', 'l'
        assert!(!has_no_forbidden_chars("i"));        // just 'i'
        assert!(!has_no_forbidden_chars("o"));        // just 'o'
        assert!(!has_no_forbidden_chars("l"));        // just 'l'
    }

    #[test]
    fn test_has_increasing_straight() {
        // Basic positive cases
        assert!(has_increasing_straight("abc")); // initial match
        assert!(has_increasing_straight("cde")); // middle match
        assert!(has_increasing_straight("xyz")); //end match

        // Positive cases with extra characters
        assert!(has_increasing_straight("xyzabc"));
        assert!(has_increasing_straight("hijklm"));

        // Negative cases
        assert!(!has_increasing_straight("acb"));
        assert!(!has_increasing_straight("aaa"));

        // Edge cases
        assert!(!has_increasing_straight("ab"));    // too short
        assert!(!has_increasing_straight(""));      // empty
        assert!(!has_increasing_straight("a"));     // single char
    }

    #[test]
    fn test_has_two_different_pairs() {
        // Basic positive cases - two different pairs
        assert!(has_two_different_pairs("aabb"));
        assert!(has_two_different_pairs("xxyyzz"));
        assert!(has_two_different_pairs("xyaabbz"));

        // Negative cases - only one pair type or no pairs
        assert!(!has_two_different_pairs("aaaa"));     // only 'a' pairs
        assert!(!has_two_different_pairs("abccde"));   // only one 'c' pair
        assert!(!has_two_different_pairs("abcdef"));   // no pairs at all

        // Edge cases
        assert!(!has_two_different_pairs(""));         // empty string
        assert!(!has_two_different_pairs("a"));        // single character
        assert!(!has_two_different_pairs("ab"));       // no pairs

        // Non-overlapping behavior
        assert!(has_two_different_pairs("aaabb"));     // aa, bb
        assert!(!has_two_different_pairs("aaab"));     // only aa pair
    }
}