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
        let (rules, sets) = parse_input(input);

        let mut sum = 0;
        let mut seen = HashSet::new();

        'set: for set in &sets {
            seen.clear();
            for num in set {
                match rules.get(num) {
                    Some(num_rules) => {
                        for rule in num_rules {
                            if seen.contains(rule) {
                                continue 'set;
                            }
                        }
                    }
                    None => (),
                }
                seen.insert(num);
            }

            sum += set.get(set.len() / 2).unwrap();
        }

        sum
    }

    fn part2(input: String) -> impl Display {
        let (rules, sets) = parse_input(input);

        let mut seen = HashSet::new();
        let mut bad_sets = vec![];

        'set: for set in &sets {
            seen.clear();
            for num in set {
                match rules.get(num) {
                    Some(num_rules) => {
                        for rule in num_rules {
                            if seen.contains(rule) {
                                bad_sets.push(set.clone());
                                continue 'set;
                            }
                        }
                    }
                    None => (),
                }
                seen.insert(num);
            }
        }

        bad_sets
            .into_iter()
            .map(|mut set| {
                set.sort_unstable_by(|a, b| {
                    rules
                        .get(a)
                        .and_then(|rule| rule.iter().find(|n| *n == b))
                        .map_or(Ordering::Greater, |_| Ordering::Less)
                });
                set.get(set.len() / 2).unwrap().clone()
            })
            .sum::<i32>()
    }
}
