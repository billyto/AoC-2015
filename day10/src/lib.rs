use std::fs::read_to_string;
use anyhow::Context;

pub fn parse_input(input_path: String) -> anyhow::Result<String> {
    let input_contents: String = read_to_string(input_path).context("Could not read input file")?;

    Ok(input_contents)
}

pub fn solve_part1(string: &String) -> usize {

    const ITERATIONS: i32 = 40;
    let mut chain = string.clone();
    for _i in 0..ITERATIONS {

        chain = look_and_say(&chain);

    }
    let length_result = chain.len();
    println!("{}", length_result);
    length_result


}

pub fn solve_part2(string: &String) -> usize {
    const ITERATIONS: i32 = 50;
    let mut chain = string.clone();
    for _i in 0..ITERATIONS {

        chain = look_and_say(&chain);

    }
    let length_result = chain.len();
    println!("{}", length_result);
    length_result
}

fn look_and_say(string: &String) -> String {

    let chars: Vec<char>  = string.chars().collect();

    let mut current_char = chars[0];
    let mut char_count = 0;
    let mut chain = String::with_capacity(string.len() * 0.3 as usize);


    for c in chars {
        // println!("{}", c);

        if c == current_char {
            char_count += 1;
        } else {
            chain.push_str(&format!("{}{}", char_count, current_char)); // this is expensive
            current_char = c;
            char_count = 1;
        }

    }
    //chain.push_str(&format!("{}{}", char_count, current_char));
    chain.push_str(&char_count.to_string());
    chain.push(current_char);
    chain

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say(){

        let chain = look_and_say(&String::from("111221"));
        assert_eq!(chain, "312211");
        // assert_eq!(chain, "1221");


    }
}

