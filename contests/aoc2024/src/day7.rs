use crate::*;
use std::fmt::Display;

impl Day7 for Year2024 {
    fn part1(input: String) -> impl Display {
        let mut sum = 0;

        for line in input.lines() {
            let (target, nums) = line.split_once(":").unwrap();

            let target = target.parse::<i64>().unwrap();

            let nums = nums
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            if try_all_combinations(&nums, &vec!['+', '*'], target) {
                sum += target;
            }
        }
        sum
    }

    fn part2(input: String) -> impl Display {
        let mut sum = 0;

        for line in input.lines() {
            let (target, nums) = line.split_once(":").unwrap();

            let target = target.parse::<i64>().unwrap();

            let nums = nums
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            if try_all_combinations(&nums, &vec!['+', '*', '|'], target) {
                sum += target;
            }
        }
        sum
    }
}

fn check_combination(nums: &Vec<i64>, target: i64, ops: &Vec<char>) -> bool {
    let mut result = nums[0];

    for i in 0..ops.len() {
        match ops[i] {
            '+' => result += nums[i + 1],
            '*' => result *= nums[i + 1],
            '|' => {
                result = (result.to_string() + &nums[i + 1].to_string())
                    .parse()
                    .unwrap()
            }
            _ => unreachable!(),
        }
    }

    result == target
}

fn try_all_combinations(nums: &Vec<i64>, ops: &Vec<char>, target: i64) -> bool {
    let n = nums.len() - 1;
    let total_combinations = ops.len().pow(n as u32);

    for i in 0..total_combinations {
        let mut cur = vec![];
        let mut num = i;

        for _ in 0..n {
            cur.push(ops[num % ops.len()]);
            num /= 3
        }

        if check_combination(nums, target, &cur) {
            return true;
        }
    }
    false
}
