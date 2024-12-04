use aoc::around::Around;
use aoc::grid::{Grid, Item};

use crate::*;
use std::fmt::Display;

const OFFSETS: [(i32, i32); 8] = [
    // north
    (0i32, -1i32),
    // south
    (0, 1),
    // east
    (1, 0),
    // west
    (-1, 0),
    // northeast
    (1, -1),
    // northwest
    (-1, -1),
    // southeast
    (1, 1),
    // southwest
    (-1, 1),
];

impl Day4 for Year2024 {
    fn part1(input: String) -> impl Display {
        find_xmas(input)
    }

    fn part2(input: String) -> impl Display {
        find_mas(input)
    }
}

fn find_xmas(input: String) -> i32 {
    let mut grid = Grid::default();

    for row in input.lines() {
        grid.add_row();

        for ch in row.chars() {
            grid.add(Item {
                label: ch,
                data: (),
            });
        }
    }

    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        let line = line.chars();

        for (j, ch) in line.enumerate() {
            if ch == 'X' {
                for offset in &OFFSETS {
                    let mut next = 'M';
                    let offset = (1..=3).map(|n| (offset.0 * n, offset.1 * n));

                    grid.visit_around(i, j, offset, |item| {
                        if item.label != next {
                            return;
                        }

                        match next {
                            'M' => next = 'A',
                            'A' => next = 'S',
                            'S' => sum += 1,
                            _ => panic!("next reached infallible state"),
                        }
                    });
                }
            }
        }
    }
    println!("{grid}");

    sum
}

fn find_mas(input: String) -> impl Display {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, line) in grid.clone().into_iter().enumerate() {
        'X: for (j, ch) in line.into_iter().enumerate() {
            if ch == 'A' {
                let offsets = vec![vec![(-1, -1), (1, 1)], vec![(1, -1), (-1, 1)]];

                let mut good = 0;
                'next: for offset in offsets {
                    let mut prev = None;
                    for corner in grid.around_offsets(i, j, offset) {
                        println!("{corner}");
                        match (prev, corner) {
                            (None, 'M' | 'S') => prev = Some(*corner),
                            (Some('M'), 'S') => good += 1,
                            (Some('S'), 'M') => good += 1,
                            _ => continue 'next,
                        }
                    }
                }

                println!("good ={good}");
                if good == 2 {
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dir_checks_south() {
        let tcs = ["...", ".X.", ".M.", ".A.", ".S."].join("\n");
        assert_eq!(find_xmas(tcs), 1);
    }

    #[test]
    fn dir_checks_east() {
        let tcs = ["......", "..XMAS", "..M...", "..A...", "..S..."].join("\n");

        assert_eq!(find_xmas(tcs), 2);
    }

    #[test]
    fn dir_checks_west() {
        let tcs = [".......", "SAMXMAS", "...M...", "...A...", "...S..."].join("\n");

        assert_eq!(find_xmas(tcs), 3);
    }

    #[test]
    fn dir_checks_north() {
        let tcs = [
            "...S...", "...A...", "...M...", "SAMXMAS", "...M...", "...A...", "...S...",
        ]
        .join("\n");

        assert_eq!(find_xmas(tcs), 4);
    }

    #[test]
    fn dir_checks_north_east() {
        let tcs = [
            "...S..S", "...A.A.", "...MM..", "SAMXMAS", "...M...", "...A...", "...S...",
        ]
        .join("\n");

        assert_eq!(find_xmas(tcs), 5);
    }

    #[test]
    fn dir_checks_north_west() {
        let tcs = [
            "S......", ".A.....", "..M....", "...XMAS", "....M..", ".....A.", "......S",
        ]
        .join("\n");

        assert_eq!(find_xmas(tcs), 3);
    }

    #[test]
    fn dir_checks_south_east() {
        let tcs = [
            "......S", ".....A.", "....M..", "...XMAS", "..M....", ".A.....", "S......",
        ]
        .join("\n");

        assert_eq!(find_xmas(tcs), 3);
    }

    #[test]
    fn dir_checks_south_west() {
        let tcs = [
            "S......", ".A.....", "..M....", "SAMX...", "....M..", ".....A.", "......S",
        ]
        .join("\n");

        assert_eq!(find_xmas(tcs), 3);
    }

    #[test]
    fn dir_checks_all_directions() {
        let tcs = [
            "S..S..S", ".A.A.A.", "..MMM..", "SAMXMAS", "..MMM..", ".A.A.A.", "S..S..S",
        ]
        .join("\n");

        assert_eq!(find_xmas(tcs), 8);
    }
}
