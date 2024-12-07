use crate::*;
use std::{collections::HashSet, fmt::Display};

impl Day6 for Year2024 {
    fn part1(input: String) -> impl Display {
        let matrix = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut cur = (0, 0);
        for (i, row) in matrix.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                if *ch == '^' {
                    cur = (i, j);
                }
            }
        }

        let mut seen = HashSet::new();
        let mut n = 1;
        let mut dir = 0;

        let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        loop {
            let di = cur.0.wrapping_add_signed(dirs[dir].0);
            let dj = cur.1.wrapping_add_signed(dirs[dir].1);

            let ch = match matrix.get(di).map(|r| r.get(dj)).flatten() {
                Some(ch) => ch,
                None => break,
            };

            if !seen.contains(&(di, dj)) && *ch != '#' {
                n += 1;
            }

            match ch {
                '#' => {
                    dir = (dir + 1) % 4;
                }
                _ => {
                    seen.insert(cur);
                    cur = (di, dj)
                }
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
