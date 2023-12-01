use std::{collections::HashSet, fs};

// beginning with frequency `0`, return the result of applying the frequency
// changes in the input string, which contain both positive and negative numbers
// separated by newlines (and a blank line at the end).
fn part_one(input: &String) -> i32 {
    input
        .trim()
        .lines()
        .map(|s| i32::from_str_radix(s, 10).unwrap_or(0))
        .sum()
}

// beginning with frequency `0`, find the first frequency that occurs twice
fn part_two(input: &String) -> i32 {
    let mut value = 0;
    let mut seen: HashSet<i32> = HashSet::from([value]);
    for line in input.trim().lines().cycle() {
        let x = i32::from_str_radix(line, 10).unwrap_or(0);
        value += x;
        if seen.contains(&value) {
            return value;
        } else {
            seen.insert(value);
        }
    }
    panic!("part_two() didn't find any frequency twice!");
}

fn main() {
    let input =
        fs::read_to_string("src/input").expect("Input file should be in the src directory!");
    println!("2018 Day01 Part01: Frequency: {:?}", part_one(&input));
    println!("2018 Day01 Part02: Frequency: {:?}", part_two(&input));
}
