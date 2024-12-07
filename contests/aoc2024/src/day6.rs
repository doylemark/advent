use crate::*;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
use Direction::*;

#[derive(Debug, PartialEq, Eq)]
struct Cursor {
    i: usize,
    j: usize,
    dir: Direction,
}

impl Cursor {
    fn forward(&mut self) {
        match self.dir {
            North => self.i = self.i.wrapping_sub(1),
            East => self.j += 1,
            South => self.i += 1,
            West => self.j = self.j.wrapping_sub(1),
        };
    }

    fn back(&mut self) {
        match self.dir {
            North => self.i += 1,
            East => self.j = self.j.wrapping_sub(1),
            South => self.i = self.i.wrapping_sub(1),
            West => self.j += 1,
        };
    }

    fn turn(&mut self) {
        self.dir = self.dir.turn()
    }
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

impl Day6 for Year2024 {
    fn part1(input: String) -> impl Display {
        let (matrix, mut cursor) = parse_input(&input);

        let mut seen = HashSet::new();
        let mut n = 0;

        while let Some(ch) = matrix.get(cursor.i).map(|r| r.get(cursor.j)).flatten() {
            if !seen.contains(&(cursor.i, cursor.j)) && *ch != '#' {
                n += 1;
            }

            seen.insert((cursor.i, cursor.j));
            match ch {
                '#' => {
                    cursor.back();
                    cursor.turn();
                }
                _ => cursor.forward(),
            }
        }

        n
    }

    fn part2(input: String) -> impl Display {
        // let (matrix, mut cursor) = parse_input(&input);
        //
        // let mut obstacles = vec![];
        //
        // while let Some(ch) = matrix.get(cursor.i).map(|r| r.get(cursor.j)).flatten() {
        //     match ch {
        //         '#' => {
        //             obstacles.push(((cursor.i, cursor.j), cursor.dir));
        //             cursor.back();
        //             cursor.turn();
        //         }
        //         _ => cursor.forward(),
        //     }
        // }

        ""
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Cursor) {
    let lines = &mut input.lines();

    let matrix: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let mut cursor = None;

    for (i, row) in matrix.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '^' {
                cursor = Some(Cursor {
                    i,
                    j,
                    dir: Direction::North,
                });
            }
        }
    }

    (matrix, cursor.expect("infallible: no cursor in input"))
}
