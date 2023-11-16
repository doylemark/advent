use std::fs;

use clap::Parser;

mod fifteen;

#[derive(Debug, Parser)]
#[command()]
struct Args {
    #[arg(short)]
    year: String,
    #[arg(short)]
    day: String,
    #[arg(short)]
    file_path: Option<String>,
    input: Option<String>,
}

enum Year {
    Fifteen,
}

impl From<String> for Year {
    fn from(value: String) -> Self {
        match value.as_str() {
            "2015" => Year::Fifteen,
            _ => panic!("Unknown year!"),
        }
    }
}

pub trait Runner {
    fn new() -> Self;
    fn run(&self, day: String, input: String) -> i32;
}

fn main() {
    let mut args = Args::parse();

    if let Some(file_path) = args.file_path {
        args.input = Some(fs::read_to_string(file_path).expect("Failed to read file"));
    }

    let res = match Year::from(args.year) {
        Year::Fifteen => fifteen::Aoc2015::new().run(args.day, args.input.expect("No input given")),
    };

    println!("{:?}", res);
}
