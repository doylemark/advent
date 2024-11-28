use crate::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Move {
    fn from(val: &str) -> Self {
        match val {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("infallible"),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Move {
    pub fn loses_to(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn outcome(self, other: Self) -> Outcome {
        if self.clone().loses_to() == other {
            Outcome::Loss
        } else if other.loses_to() == self {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }
}

impl Day2 for Year2022 {
    fn part1(input: String) -> impl Display {
        for line in input.lines() {
            let mut words = line.split_whitespace();
            let (opponent, own): (Move, Move) =
                (words.next().unwrap().into(), words.next().unwrap().into());

            println!("{:#?}", own.outcome(opponent));
        }

        input
    }

    fn part2(input: String) -> impl Display {
        input
    }
}
