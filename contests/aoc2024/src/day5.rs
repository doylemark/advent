use crate::*;
use std::{collections::HashMap, fmt::Display};

impl Day5 for Year2024 {
    fn part1(input: String) -> impl Display {
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

        let mut sum = 0;

        'set: for set in sets {
            for (i, num) in set.iter().enumerate() {
                match rules.get(num) {
                    Some(num_rules) => {
                        let mut num_rules = num_rules.iter();

                        while let Some(rule) = num_rules.next() {
                            for j in (0..i).rev() {
                                if set[j] == *rule {
                                    continue 'set;
                                }
                            }
                        }
                    }
                    None => continue,
                }
            }

            sum += set.get(set.len() / 2).unwrap();
        }

        sum
    }

    fn part2(input: String) -> impl Display {
        input
    }
}
