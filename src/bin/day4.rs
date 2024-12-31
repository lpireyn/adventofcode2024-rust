#! https: //adventofcode.com/2024/day/4

use std::fs;

type Puzzle = Vec<Vec<char>>;

#[derive(Debug)]
struct PuzzleIter<'a> {
    puzzle: &'a Puzzle,
    dx: i8,
    dy: i8,
    x: i16,
    y: i16,
}

impl Iterator for PuzzleIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < 0 || self.y < 0 {
            return None;
        }
        if let Some(row) = self.puzzle.get(self.y as usize) {
            if let Some(&c) = row.get(self.x as usize) {
                self.x += self.dx as i16;
                self.y += self.dy as i16;
                Some(c)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn main() {
    // Read puzzle
    let puzzle: Puzzle = fs::read_to_string("input/day4.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Count occurrences of "XMAS" in puzzle
    let xmas = "XMAS";
    let mut part1: u16 = 0;
    for y in 0..puzzle.len() {
        let row = &puzzle[y];
        for x in 0..row.len() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let word = PuzzleIter {
                        puzzle: &puzzle,
                        dx,
                        dy,
                        x: x as i16,
                        y: y as i16,
                    }
                    .take(xmas.chars().count())
                    .collect::<String>();
                    if word == xmas {
                        part1 += 1;
                    }
                }
            }
        }
    }

    println!("Part one: {}", &part1);
    assert_eq!(2397, part1);

    // Count occurrences of X-MAS in puzzle
    let mut part2: u16 = 0;
    for y in 1..puzzle.len() - 1 {
        let row = &puzzle[y];
        for x in 1..(row.len() - 1) {
            let c = row[x];
            let nw = puzzle[y - 1][x - 1];
            let ne = puzzle[y - 1][x + 1];
            let sw = puzzle[y + 1][x - 1];
            let se = puzzle[y + 1][x + 1];
            if c == 'A'
                && ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M'))
                && ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M'))
            {
                part2 += 1;
            }
        }
    }

    println!("Part two: {}", &part2);
    assert_eq!(1824, part2);
}
