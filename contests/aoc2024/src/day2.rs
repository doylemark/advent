use std::fmt::Display;

use crate::*;

impl Day2 for Year2024 {
    fn part1(input: String) -> impl Display {
        input
            .lines()
            .filter_map(|line| {
                let nums = line
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                nums.windows(2)
                    .all(|w| w[0].cmp(&w[1]) == nums[0].cmp(&nums[1]) && w[0].abs_diff(w[1]) <= 3)
                    .then_some(())
            })
            .count()
    }

    fn part2(input: String) -> impl Display {
        let mut sum = 0;

        for line in input.lines() {
            let mut nums = line
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap());
        }

        sum
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    Idle,
    Terminated,
    Asc,
    Desc,
}

#[derive(Debug)]
struct FSM {
    inner: State,
    transitions: i32,
}

impl FSM {
    fn new() -> Self {
        Self {
            inner: State::Idle,
            transitions: 0,
        }
    }

    fn state(&self) -> &State {
        &self.inner
    }

    fn transition(&mut self, new: State) {
        println!("new=[{new:?}]");
        self.inner = new;
    }

    fn next(&mut self, prev: i32, n: i32) {
        let delta = prev - n;
    }
}
