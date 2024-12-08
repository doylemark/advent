use crate::*;
use std::fmt::Display;

fn parse_input(input: &str) -> impl Iterator<Item = (i64, Vec<i64>)> + '_ {
    input.lines().map(|l| {
        let (target, nums) = l.split_once(":").unwrap();

        let target = target.parse::<i64>().unwrap();

        let nums = nums
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        (target, nums)
    })
}

impl Day7 for Year2024 {
    fn part1(input: String) -> impl Display {
        parse_input(&input)
            .map(|(target, nums)| try_all_combinations_1(&nums, target))
            .sum::<i64>()
    }

    fn part2(input: String) -> impl Display {
        parse_input(&input)
            .map(|(target, nums)| try_all_combinations(&nums, target))
            .sum::<i64>()
    }
}

fn try_all_combinations_1(nums: &[i64], target: i64) -> i64 {
    let n = nums.len() - 1;
    let total_combinations = 1 << n;

    for i in 0..total_combinations {
        let mut result = nums[0];

        for pos in 0..n {
            if i & (1 << pos) != 0 {
                result += nums[pos + 1];
            } else {
                result *= nums[pos + 1];
            }
        }

        if result == target {
            return target;
        }
    }
    0
}

fn try_all_combinations(nums: &[i64], target: i64) -> i64 {
    let n = nums.len() - 1;
    let total_combinations = 3_i64.pow(n as u32);

    let mut ops = vec![0u8; n];

    for i in 0..total_combinations {
        let mut num = i;

        for j in 0..n {
            ops[j] = (num % 3) as u8;
            num /= 3;
        }

        if check_combination(nums, target, &ops) {
            return target;
        }
    }
    0
}

fn check_combination(nums: &[i64], target: i64, ops: &[u8]) -> bool {
    let mut result = nums[0];

    for i in 0..ops.len() {
        match ops[i] {
            0 => result += nums[i + 1],
            1 => result *= nums[i + 1],
            2 => {
                let mut power = 10;
                let mut temp = nums[i + 1];
                while temp >= 10 {
                    power *= 10;
                    temp /= 10;
                }
                result = result * power + nums[i + 1];
            }
            _ => unreachable!(),
        }

        if result > target {
            return false;
        }
    }

    result == target
}
