use std::fmt::Display;

use crate::*;

impl Day2 for Year2023 {
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    fn part1(input: String) -> impl Display {
        let mut sum = 0;
        for (_, line) in input.lines().enumerate() {
            let mut red_max = 0;
            let mut blue_max = 0;
            let mut green_max = 0;

            for go in line.split(";") {
                for color in go.split(",") {
                    let mut words = color.split_whitespace();
                    let (n, color) = (
                        words.next().unwrap().parse::<i32>().unwrap(),
                        words.next().unwrap(),
                    );

                    if color == "red" && n > red_max {
                        red_max = n;
                    }

                    if color == "green" && n > green_max {
                        green_max = n;
                    }

                    if color == "blue" && n > blue_max {
                        blue_max = n;
                    }
                }
            }

            sum += red_max * blue_max * green_max
        }

        sum.to_string()
    }

    fn part2(input: String) -> impl Display {
        input
    }
}
