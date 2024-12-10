use crate::*;
use std::{array::from_fn, collections::HashSet, fmt::Display};

impl Day10 for Year2024 {
    fn part1(input: String) -> impl Display {
        let grid: Vec<Vec<i8>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|n| n.to_string().parse().unwrap_or(-1))
                    .collect()
            })
            .collect();

        let h = grid.len();
        let w = grid[0].len();

        let mut set = HashSet::new();
        let mut sum = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if *n == 0 {
                    sum += dfs(&grid, &mut set, i as isize, j as isize, 0, h, w);
                    set.clear();
                }
            }
        }

        sum
    }

    fn part2(input: String) -> impl Display {
        let grid: Vec<Vec<i8>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|n| n.to_string().parse().unwrap_or(-1))
                    .collect()
            })
            .collect();
        let h = grid.len();
        let w = grid[0].len();

        let mut sum = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if *n == 0 {
                    sum += dfs2(&grid, i as isize, j as isize, 0, h, w);
                }
            }
        }

        sum
    }
}

fn dfs(
    grid: &Vec<Vec<i8>>,
    res: &mut HashSet<(isize, isize)>,
    i: isize,
    j: isize,
    target: i8,
    h: usize,
    w: usize,
) -> i32 {
    if i < 0 || j < 0 || i >= h as isize || j >= w as isize {
        return 0;
    }

    if grid[i as usize][j as usize] != target {
        return 0;
    }

    if target == 9 && !res.contains(&(i, j)) {
        res.insert((i, j));
        return 1;
    }

    return dfs(grid, res, i - 1, j, target + 1, h, w)
        + dfs(grid, res, i + 1, j, target + 1, h, w)
        + dfs(grid, res, i, j + 1, target + 1, h, w)
        + dfs(grid, res, i, j - 1, target + 1, h, w);
}

fn dfs2(grid: &Vec<Vec<i8>>, i: isize, j: isize, target: i8, h: usize, w: usize) -> i32 {
    if i < 0 || j < 0 || i >= h as isize || j >= w as isize {
        return 0;
    }

    if grid[i as usize][j as usize] != target {
        return 0;
    }

    if target == 9 {
        return 1;
    }

    return dfs2(grid, i - 1, j, target + 1, h, w)
        + dfs2(grid, i + 1, j, target + 1, h, w)
        + dfs2(grid, i, j + 1, target + 1, h, w)
        + dfs2(grid, i, j - 1, target + 1, h, w);
}
