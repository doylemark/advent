use core::panic;
use std::collections::HashMap;

use crate::Runner;

pub struct Aoc2023;

impl Runner for Aoc2023 {
    fn new() -> Self {
        Self
    }

    fn run(&self, day: String, input: String) -> i32 {
        match day.as_str() {
            "1" => day_one(input),
            "1.1" => day_one_2(input),
            "2" => day_two(input),
            _ => panic!("unknown day!"),
        }
    }
}

fn day_one(input: String) -> i32 {
    input
        .split("\n")
        .map(|ln| {
            let digits = ln.chars().filter(|ch| ch.is_digit(10));

            (digits.clone().rev().last().unwrap_or_default().to_string()
                + digits.last().unwrap_or_default().to_string().as_str())
            .parse::<i32>()
            .unwrap()
        })
        .sum()
}

fn day_one_2(input: String) -> i32 {
    let digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    input
        .split("\n")
        .map(|ln| {
            let mut nums = vec![];
            let chars = ln.chars().collect::<Vec<char>>();

            for i in 0..ln.len() + 1 {
                if let Some(n) = chars.get(i) {
                    if n.is_digit(10) {
                        nums.push(n.to_string().parse::<i32>().unwrap())
                    }
                };

                for j in i..ln.len() + 1 {
                    if let Some(digit) = digits.get(&ln[i..j]) {
                        nums.push(*digit);
                    }
                }
            }

            vec![nums.first().unwrap_or(&0), nums.last().unwrap_or(&0)]
                .iter()
                .fold("".to_string(), |ctx, n| ctx + n.to_string().as_str())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

struct Color {
    r: i32,
    g: i32,
    b: i32,
}

fn day_two(input: String) -> i32 {
    fn color_quantities(s: String) -> Color {
        let mut c = Color { r: 0, g: 0, b: 0 };

        let mut current_n = "".to_string();

        for ch in s.chars() {
            if ch.is_digit(10) {
                current_n += ch.to_string().as_str();
            }

            if ch.is_alphabetic() {
                if current_n.is_empty() {
                    continue;
                }

                let n = current_n.parse::<i32>().unwrap();
                current_n = "".to_string();

                match ch {
                    'r' => c.r += n,
                    'g' => c.g += n,
                    'b' => c.b += n,
                    _ => panic!("unknown color"),
                }
            }
        }

        c
    }

    input
        .split("\n")
        .filter_map(|ln| {
            let parts = ln.split(":").collect::<Vec<&str>>();

            let game_id = parts
                .first()
                .expect("missing game id")
                .trim_start_matches("Game ")
                .parse::<i32>()
                .expect("invalid game id");

            let handfuls = parts.last().expect("missing handfuls").split(";");

            if handfuls
                .clone()
                .map(|hf| color_quantities(hf.to_string()))
                .all(|c| c.r <= 12 && c.g <= 13 && c.b <= 14)
            {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}
