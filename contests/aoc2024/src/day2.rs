use std::fmt::Display;

use crate::*;

impl Day2 for Year2024 {
    fn part1(input: String) -> impl Display {
        let mut sum = 0;

        for line in input.lines() {
            let nums = line
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if is_valid_line(&nums) {
                sum += 1;
            }
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
                .collect::<Vec<_>>();

            let input = nums.clone();

            if is_valid_line(&nums) {
                sum += 1;
            } else {
                for i in 0..input.len() {
                    let mut removed = input.clone();
                    removed.remove(i);

                    if is_valid_line(&removed) {
                        sum += 1;
                        break;
                    }
                }
            }
        }

        sum
    }
}

fn is_valid_line(nums: &Vec<i32>) -> bool {
    nums.windows(2)
        .all(|w| w[0].cmp(&w[1]) == nums[0].cmp(&nums[1]) && w[0].abs_diff(w[1]) <= 3)
}
