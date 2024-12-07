use std::time::Instant;

use aoc2021::run as run2021;
use aoc2022::run as run2022;
use aoc2023::run as run2023;
use aoc2024::run as run2024;

mod cmd;

fn main() {
    let (year, day, part) = cmd::parse_cmd(
        std::env::args()
            .collect::<Vec<_>>()
            .get(1)
            .expect("no command"),
    );

    let before = Instant::now();
    let input = std::fs::read_to_string("./input.txt").expect("failed to read input");

    let output = match year {
        24 => run2024(day, part, input),
        23 => run2023(day, part, input),
        22 => run2022(day, part, input),
        21 => run2021(day, part, input),
        _ => panic!("year not implemented"),
    };

    println!("{output}");
    println!("Elapsed time: {:.2?}", before.elapsed());
}
