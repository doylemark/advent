use core::panic;

use crate::Runner;

pub struct Aoc2023;

mod day1;
mod day2;
mod day3;
mod day4;

impl Runner for Aoc2023 {
    fn new() -> Self {
        Self
    }

    fn run(&self, day: String, input: String) -> i32 {
        match day.as_str() {
            "1" => day1::day_1(input),
            "1.1" => day1::day_1_2(input),
            "2" => day2::day_2(input),
            "2.1" => day2::day_2_2(input),
            "3" => day3::day_3(input),
            // "3.1" => day_3_2(input),
            "4" => day4::day_four(input),
            _ => panic!("unknown day!"),
        }
    }
}
