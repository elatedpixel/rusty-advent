use std::{collections::HashSet, fs};

// beginning with frequency `0`, return the result of applying the frequency
// changes in the input string, which contain both positive and negative numbers
// separated by newlines (and a blank line at the end).
fn part_one(input: &str) -> i32 {
    input.trim().lines().map(parse_frequency_change).sum()
}

// beginning with frequency `0`, find the first frequency that occurs twice
fn part_two(input: &str) -> i32 {
    let mut frequency = 0;
    let mut seen: HashSet<i32> = HashSet::from([frequency]);
    for change in input.trim().lines().map(parse_frequency_change).cycle() {
        frequency += change;
        if seen.contains(&frequency) {
            return frequency;
        } else {
            seen.insert(frequency);
        }
    }
    panic!("part_two() didn't find any frequency twice!");
}

fn parse_frequency_change(change_string: &str) -> i32 {
    change_string
        .parse::<i32>()
        .expect("All change strings should contain integers!")
}

fn main() {
    let input =
        fs::read_to_string("src/input").expect("Input file should be in the src directory!");
    println!("2018 Day01 Part01: Frequency: {}", part_one(&input));
    println!("2018 Day01 Part02: Frequency: {}", part_two(&input));
}
