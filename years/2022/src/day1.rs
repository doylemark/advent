use std::fmt::Display;

use crate::*;

impl Day1 for Year2022 {
    fn part1(input: String) -> impl Display {
        let mut max = 0;
        let mut rolling_sum = 0;

        for line in input.lines() {
            let line = line.trim();

            if line.is_empty() {
                if rolling_sum > max {
                    max = rolling_sum;
                }
                rolling_sum = 0;
                continue;
            }

            rolling_sum += line.parse::<i32>().unwrap();
        }

        max
    }

    fn part2(input: String) -> impl Display {
        let mut sums = vec![];
        let mut rolling_sum = 0;

        for line in input.lines() {
            let line = line.trim();

            if line.is_empty() {
                sums.push(rolling_sum);
                rolling_sum = 0;
                continue;
            }

            rolling_sum += line.parse::<i32>().unwrap();
        }

        sums.sort_by(|a, b| b.cmp(a));
        sums.iter().take(3).sum::<i32>()
    }
}
