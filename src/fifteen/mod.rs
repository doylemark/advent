use crate::Runner;

mod day_one;

pub struct Aoc2015;

impl Runner for Aoc2015 {
    fn new() -> Self {
        Self
    }

    fn run(&self, day: String) -> String {
        match day.as_str() {
            "1" => day_one::day_one(),
            _ => panic!("unknown day!"),
        }
    }
}
