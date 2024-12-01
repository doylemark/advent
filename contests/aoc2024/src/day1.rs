use std::{collections::HashMap, fmt::Display};

use crate::*;

impl Day1 for Year2024 {
    fn part1(input: String) -> impl Display {
        let mut a = vec![];
        let mut b = vec![];

        for line in input.lines() {
            let mut words = line.split_whitespace();
            let (first, second) = (words.next().unwrap(), words.next().unwrap());

            a.push(first.parse::<i32>().unwrap());
            b.push(second.parse::<i32>().unwrap());
        }

        a.sort();
        b.sort();

        let mut sum = 0;
        for i in 0..a.len() {
            let delta = a[i] - b[i];
            sum += delta.abs();
        }

        sum
    }

    fn part2(input: String) -> impl Display {
        let mut a = vec![];
        let mut b = HashMap::new();

        for line in input.lines() {
            let mut words = line.split_whitespace();
            let (first, second) = (words.next().unwrap(), words.next().unwrap());

            a.push(first.parse::<i32>().unwrap());
            b.entry(second.parse::<i32>().unwrap())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let mut sum = 0;

        for n in a {
            if let Some(c) = b.get(&n) {
                sum += n * c
            }
        }

        sum
    }
}
