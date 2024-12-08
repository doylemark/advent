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
        let ops = vec!['+', '*', '|'];
        parse_input(&input)
            .map(|(target, nums)| try_all_combinations(&nums, &ops, target))
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
fn try_all_combinations(nums: &Vec<i64>, ops: &Vec<char>, target: i64) -> i64 {
    let n = nums.len() - 1;
    let total_combinations = ops.len().pow(n as u32);

    for i in 0..total_combinations {
        let mut cur = vec![];
        let mut num = i;

        for _ in 0..n {
            cur.push(ops[num % ops.len()]);

            num /= ops.len()
        }

        if check_combination(nums, target, &cur) {
            return target;
        }
    }
    0
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
