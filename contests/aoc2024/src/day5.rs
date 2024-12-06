use crate::*;
use std::{cmp::Ordering, collections::HashSet, fmt::Display};

impl Day5 for Year2024 {
    fn part1(input: String) -> impl Display {
        let lines = &mut input.lines();

        let rules = lines
            .take_while(|l| !l.is_empty())
            .map(|l| l.split_once("|").unwrap())
            .collect::<HashSet<_>>();

        lines
            .filter_map(|line| {
                let nums = line.split(",").collect::<Vec<_>>();

                nums.is_sorted_by(|&a, &b| rules.contains(&(a, b)))
                    .then(|| nums[nums.len() / 2].parse::<i32>().unwrap())
            })
            .sum::<i32>()
    }

    fn part2(input: String) -> impl Display {
        let lines = &mut input.lines();

        let rules = lines
            .take_while(|l| !l.is_empty())
            .map(|l| l.split_once("|").unwrap())
            .collect::<HashSet<_>>();

        lines
            .filter_map(|line| {
                let mut nums = line.split(",").collect::<Vec<_>>();

                (!nums.is_sorted_by(|&a, &b| rules.contains(&(a, b)))).then(|| {
                    nums.sort_by(|a, b| {
                        if rules.contains(&(a, b)) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    });
                    nums[nums.len() / 2].parse::<i32>().unwrap()
                })
            })
            .sum::<i32>()
    }
}
