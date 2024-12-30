#! https: //adventofcode.com/2024/day/3

use regex::Regex;
use std::fs;

fn main() {
    // Read input
    let input = fs::read_to_string("input/day3.txt").unwrap();

    // Sum the valid products in input
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let part1: u32 = pattern
        .captures_iter(&input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum();

    println!("Part one: {}", &part1);
    assert_eq!(187194524, part1);

    let pattern = Regex::new(r"do(?:n't)?\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut part2: u32 = 0;
    let mut to_do = true;
    pattern.captures_iter(&input).for_each(|c| {
        let m = &c[0];
        if m.starts_with("do(") {
            to_do = true;
        } else if m.starts_with("don") {
            to_do = false;
        } else {
            assert!(m.starts_with("mul"));
            if to_do {
                part2 += c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap();
            }
        }
    });

    println!("Part two: {}", &part2);
    assert_eq!(0, part2);
}
