use std::collections::{HashMap, HashSet};

use crate::Runner;

pub struct Aoc2015;

impl Runner for Aoc2015 {
    fn new() -> Self {
        Self
    }

    fn run(&self, day: String, input: String) -> i32 {
        match day.as_str() {
            "1" => day_one(input),
            "1.1" => day_one_2(input),
            "2" => day_two(input),
            "2.1" => day_two_2(input),
            "3" => day_three(input),
            "3.1" => day_three_2(input),
            _ => panic!("unknown day!"),
        }
    }
}

fn day_one(input: String) -> i32 {
    let mut open_count = 0;
    let mut close_count = 0;

    for char in input.chars() {
        if char == '(' {
            open_count += 1;
        } else {
            close_count += 1;
        }
    }

    open_count - close_count
}

fn day_one_2(input: String) -> i32 {
    let mut open_count = 0;
    let mut close_count = 0;

    for (i, char) in input.chars().enumerate() {
        if char == '(' {
            open_count += 1;
        } else {
            close_count += 1;
        }

        if open_count - close_count == -1 {
            return (i + 1).try_into().expect("failed to convert index to i32");
        }
    }

    open_count - close_count
}

fn day_two(input: String) -> i32 {
    let mut total: i32 = 0;

    for line in input.lines() {
        let dims: Vec<i32> = line
            .split("x")
            .map(|n| n.trim().parse().expect("invalid dimension"))
            .collect();

        let l = dims.get(0).expect("Missing len");
        let w = dims.get(1).expect("Missing width");
        let h = dims.get(2).expect("Missing h");

        let top_size = l * w;
        let side_size = w * h;
        let front_back_size = h * l;

        let size = 2 * top_size + 2 * side_size + 2 * front_back_size;

        let excess = top_size.min(side_size).min(front_back_size);

        total += size + excess;
    }

    total
}

fn day_two_2(input: String) -> i32 {
    let mut total: i32 = 0;

    for line in input.lines() {
        let dims: Vec<i32> = line
            .split("x")
            .map(|n| n.trim().parse().expect("invalid dimension"))
            .collect();

        let l = dims.get(0).expect("Missing len");
        let w = dims.get(1).expect("Missing width");
        let h = dims.get(2).expect("Missing h");

        let top_perim = l * 2 + w * 2;
        let side_perim = w * 2 + h * 2;
        let front_back_permit = h * 2 + l * 2;

        let min_perim = top_perim.min(side_perim).min(front_back_permit);

        let vol = l * w * h;

        total += vol + min_perim;
    }

    total
}

fn day_three(input: String) -> i32 {
    let mut seen = HashMap::<(i32, i32), i32>::new();

    input
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .fold((0, 0), |ctx, ch| {
            let opr = match ch {
                '^' => (0, 1),
                '>' => (1, 0),
                '<' => (-1, 0),
                'v' => (0, -1),
                _ => panic!("unknown operator"),
            };

            let new_coords = (opr.0 + ctx.0, opr.1 + ctx.1);
            seen.insert(new_coords, seen.get(&new_coords).unwrap_or(&0) + 1);

            new_coords
        });

    seen.values()
        .filter(|n| **n > 0)
        .copied()
        .collect::<Vec<i32>>()
        .len() as i32
}

fn day_three_2(input: String) -> i32 {
    let mut grid = HashSet::new();

    let mut x = [0, 0];
    let mut y = [0, 0];
    let mut which = 0;

    for ch in input.chars() {
        grid.insert((x[which], y[which]));
        match ch {
            '^' => y[which] += 1,
            '>' => x[which] += 1,
            '<' => x[which] -= 1,
            'v' => y[which] -= 1,
            _ => panic!("unknown input"),
        };

        which = 1 - which;
    }
    grid.insert((x[which], y[which]));

    grid.len().try_into().expect("invalid grid len")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_one_base_case() {
        assert_eq!(day_one("()()".to_string()), 0);
    }

    #[test]
    fn day_one_level_negative_3() {
        assert_eq!(day_one(")())())".to_string()), -3);
        assert_eq!(day_one(")))".to_string()), -3);
    }

    #[test]
    fn day_one_level_3() {
        assert_eq!(day_one("))(((((".to_string()), 3);
        assert_eq!(day_one("(()(()(".to_string()), 3);
    }

    #[test]
    fn day_one_2_base_case() {
        assert_eq!(day_one_2(")".to_string()), 1);
        assert_eq!(day_one_2("()())".to_string()), 5);
    }

    #[test]
    fn day_two_base_case() {
        assert_eq!(day_two("2x3x4".to_string()), 58);
    }

    #[test]
    fn day_two_2_base_case() {
        assert_eq!(day_two_2("2x3x4".to_string()), 34);
    }

    #[test]
    fn day_three_base_case() {
        assert_eq!(day_three(">".to_string()), 1);
        assert_eq!(day_three("^>v<".to_string()), 4);
        assert_eq!(day_three("^v^v^v^v^v".to_string()), 2);
    }
}
