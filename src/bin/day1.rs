#! https: //adventofcode.com/2024/day/1

use std::fs;

type N = u32;

fn main() {
    // Read both lists of numbers
    let (mut ns1, mut ns2): (Vec<N>, Vec<N>) = fs::read_to_string("input/day1.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            (
                tokens.next().unwrap().parse::<N>().unwrap(),
                tokens.next().unwrap().parse::<N>().unwrap(),
            )
        })
        .unzip();

    // Sort each list of numbers
    ns1.sort();
    ns2.sort();

    // Sum the differences between each pair of numbers
    let part1: N = ns1
        .iter()
        .zip(ns2.iter())
        .map(|(n1, n2)| n1.abs_diff(*n2))
        .sum();

    println!("Part one: {}", &part1);
    assert_eq!(1506483, part1);

    // Sum the counts of occurrences in second list of each number in first list
    let part2: N = ns1
        .iter()
        .map(|n| {
            let count = ns2.iter().filter(|n2| *n2 == n).count();
            count as N * n
        })
        .sum();

    println!("Part two: {}", &part2);
    assert_eq!(23126924, part2);
}
