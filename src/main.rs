use clap::Parser;

mod fifteen;

#[derive(Debug, Parser)]
#[command()]
struct Args {
    #[arg(short)]
    year: String,
    #[arg(short)]
    day: String,
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
    fn run(&self, day: String) -> String;
}

fn main() {
    let args = Args::parse();

    let res = match Year::from(args.year) {
        Year::Fifteen => fifteen::Aoc2015::new().run(args.day),
    };

    println!("{:?}", res);
}
