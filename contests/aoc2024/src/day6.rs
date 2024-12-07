use crate::*;
use std::{collections::HashSet, fmt::Display};

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
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

    (matrix, cur)
}

impl Day6 for Year2024 {
    fn part1(input: String) -> impl Display {
        let (matrix, mut cur) = parse_input(&input);

        let mut seen = vec![];

        for row in &matrix {
            for _ in row {
                seen.push(false);
            }
        }
        let width = matrix.get(0).unwrap().len();

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

            if !seen[di * width + dj] && *ch != '#' {
                n += 1;
            }

            if '#' == *ch {
                dir = (dir + 1) % 4;
            } else {
                seen[di * width + dj] = true;
                cur = (di, dj)
            }
        }

        n
    }

    fn part2(input: String) -> impl Display {
        let (mut matrix, start) = parse_input(&input);
        let mut n = 0;
        let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if (i, j) == start || matrix[i][j] != '.' {
                    continue;
                }

                matrix[i][j] = '#';

                let mut cur = start;
                let mut dir = 0;
                let mut seen = vec![];

                for row in &matrix {
                    for _ in row {
                        seen.push(0);
                    }
                }
                let width = matrix.get(0).unwrap().len();

                while let Some(Some(ch)) = matrix
                    .get(cur.0.wrapping_add_signed(dirs[dir].0))
                    .map(|r| r.get(cur.1.wrapping_add_signed(dirs[dir].1)))
                {
                    let hash = (cur.0 * width + cur.1) * 4 + dir;
                    if seen[cur.0 * width + cur.1] == hash {
                        n += 1;
                        break;
                    }
                    seen[cur.0 * width + cur.1] = hash;

                    if *ch == '#' {
                        dir = (dir + 1) % 4;
                    } else {
                        cur = (
                            cur.0.wrapping_add_signed(dirs[dir].0),
                            cur.1.wrapping_add_signed(dirs[dir].1),
                        );
                    }
                }

                matrix[i][j] = '.';
            }
        }

        n
    }
}
