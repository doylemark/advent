use std::fmt::Display;

use crate::*;

impl Day1 for Year2021 {
    fn part1(input: String) -> impl Display {
        let mut n = 0;
        let mut lines = input.lines();
        let mut prev = lines.next().unwrap().parse::<i32>().unwrap();

        for line in lines {
            let cur = line.parse::<i32>().unwrap();
            if cur > prev {
                n += 1;
            }
            prev = cur;
        }

        n.to_string()
    }

    fn part2(input: String) -> impl Display {
        input
    }
}
