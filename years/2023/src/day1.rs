use std::collections::HashMap;

use crate::*;

impl Day1 for Year2023 {
    fn part1(input: String) -> String {
        let mut sum = 0;
        for line in input.lines() {
            let mut digits = vec![];
            for ch in line.chars().into_iter() {
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }

            let mut last = digits.get(0).unwrap().to_string();
            last.push_str(&digits.last().unwrap().to_string());
            sum += last.parse::<i32>().unwrap();
        }

        return sum.to_string();
    }

    fn part2(input: String) -> String {
        let mut sum = 0;
        for line in input.lines() {
            let mut digits = vec![];
            for (i, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    digits.push(ch.to_string());
                }

                for (d, word) in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .iter()
                .enumerate()
                {
                    if line[i..].starts_with(word) {
                        digits.push((d + 1).to_string())
                    }
                }
            }

            let mut last = digits.get(0).unwrap().to_owned();
            last.push_str(digits.last().unwrap());
            sum += last.parse::<i32>().unwrap();
        }

        return sum.to_string();
    }
}
