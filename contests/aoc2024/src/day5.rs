use crate::*;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
};

fn parse_input(input: String) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules = HashMap::<i32, Vec<i32>>::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut nums = line.split("|");
        let (num, rule) = (
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
        );

        rules
            .entry(num)
            .and_modify(|v| v.push(rule))
            .or_insert(vec![rule]);
    }

    let sets = lines
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, sets)
}

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
