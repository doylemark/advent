use aoc::around::{Around, OFFSETS};

use crate::*;
use std::fmt::Display;

impl Day4 for Year2024 {
    fn part1(input: String) -> impl Display {
        find_xmas(input)
    }

    fn part2(input: String) -> impl Display {
        find_mas(input)
    }
}

fn find_xmas(input: String) -> i32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, line) in grid.clone().into_iter().enumerate() {
        for (j, ch) in line.into_iter().enumerate() {
            if ch == 'X' {
                for offset in &OFFSETS {
                    let mut next = 'M';
                    let offset = (1..=3).map(|n| (offset.0 * n, offset.1 * n));

                    for adj in grid.around_offsets(i, j, offset.collect()) {
                        if *adj != next {
                            continue;
                        }

                        match next {
                            'M' => next = 'A',
                            'A' => next = 'S',
                            'S' => sum += 1,
                            _ => panic!("next reached infallible state"),
                        }
                    }
                }
            }
        }
    }

    sum
}

fn find_mas(input: String) -> impl Display {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, line) in grid.clone().into_iter().enumerate() {
        for (j, ch) in line.into_iter().enumerate() {
            if ch == 'A' {
                let offsets = vec![(-1, -1), (1, 1), (1, -1), (-1, 1)];

                let mut seen = 0;
                let mut prev = None;

                for corner in grid.around_offsets(i, j, offsets) {
                    match (prev, corner) {
                        (None, 'M' | 'S') => prev = Some(*corner),
                        (Some('M'), 'S') | (Some('S'), 'M') => {
                            seen += 1;
                            prev = None
                        }
                        _ => break,
                    }
                }

                if seen == 2 {
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
