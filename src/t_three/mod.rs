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
