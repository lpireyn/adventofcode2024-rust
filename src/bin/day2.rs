#! https: //adventofcode.com/2024/day/2

use std::fs;

type N = i16;
type Report = Vec<N>;

fn same_sign(n1: N, n2: N) -> bool {
    (n1 >= 0 && n2 >= 0) || (n1 <= 0 && n2 <= 0)
}

fn safe_diff(d: N, pd: N) -> bool {
    same_sign(d, pd) && (1..=3).contains(&d.abs())
}

fn is_safe(report: &Report) -> bool {
    let mut pd: N = 0;
    for i in 1..report.len() {
        let d = report[i] - report[i - 1];
        if !safe_diff(d, pd) {
            return false;
        }
        pd = d;
    }
    true
}

fn is_safe_dampened(report: &Report) -> bool {
    is_safe(report)
        || (0..report.len())
            .map(|i| {
                // Map each index to a clone of the report minus the element at index
                let mut r = report.clone();
                r.remove(i);
                r
            })
            .any(|r| is_safe(&r))
}

fn main() {
    // Read reports
    let reports: Vec<Report> = fs::read_to_string("input/day2.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|token| token.parse::<N>().unwrap())
                .collect()
        })
        .collect();

    // Count the safe reports
    let part1 = reports.iter().filter(|report| is_safe(report)).count();

    println!("Part one: {}", &part1);
    assert_eq!(246, part1);

    // Count the safe reports, dampened
    let part2 = reports
        .iter()
        .filter(|report| is_safe_dampened(report))
        .count();

    println!("Part two: {}", &part2);
    assert_eq!(318, part2);
}
