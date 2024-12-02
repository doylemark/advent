use std::fmt::Display;

use crate::*;

impl Day2 for Year2024 {
    fn part1(input: String) -> impl Display {
        let mut sum = 0;

        'line: for line in input.lines() {
            let mut words = line.trim().split_whitespace().peekable();
            let mut prev = words.next().unwrap().parse::<i32>().unwrap();

            while let Some(word) = words.next() {
                let n = word.parse::<i32>().unwrap();
                let delta = prev - n;

                if delta.abs() < 1 || delta.abs() > 3 {
                    continue 'line;
                }

                if let Some(next) = words.peek() {
                    let next = next.parse::<i32>().unwrap();
                    if (delta < 0 && next < n) || (delta > 0 && next > n) {
                        continue 'line;
                    }
                }

                prev = n;
            }

            sum += 1;
        }

        sum
    }

    fn part2(input: String) -> impl Display {
        let mut sum = 0;

        for line in input.lines() {
            let nums = line
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .peekable();

            let input = nums.clone();

            if is_valid_line(nums) {
                sum += 1;
            } else {
                let input = input.collect::<Vec<_>>();

                for i in 0..input.len() {
                    let mut removed = input.clone();
                    removed.remove(i);

                    if is_valid_line(removed.into_iter()) {
                        sum += 1;
                        break;
                    }
                }
            }
        }

        sum
    }
}

fn is_valid_line<I>(words: I) -> bool
where
    I: Iterator<Item = i32>,
{
    let mut words = words.peekable();
    let mut prev = words.next().unwrap();

    while let Some(n) = words.next() {
        let delta = prev - n;

        if delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }

        if let Some(next) = words.peek() {
            if (delta < 0 && *next < n) || (delta > 0 && *next > n) {
                return false;
            }
        }

        prev = n;
    }

    true
}
