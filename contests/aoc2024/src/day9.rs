use crate::*;
use std::fmt::Display;

impl Day9 for Year2024 {
    fn part1(input: String) -> impl Display {
        let mut chars = input.trim().chars().map(|ch| ch.to_string()).peekable();
        let mut res = vec![];

        let mut file_id = 0;
        while let Some(size) = chars.next() {
            let size = size.parse().unwrap();
            let empty = chars
                .peek()
                .map(|empty| empty.parse().unwrap())
                .unwrap_or(0);

            for _ in 0..size {
                res.push(file_id.to_string())
            }

            for _ in 0..empty {
                res.push(".".to_string());
            }
            chars.next();
            file_id += 1;
        }

        let mut i = res
            .iter()
            .position(|s| *s == ".")
            .map(|idx| idx)
            .expect("no empty ch in string");

        let mut j = res
            .iter()
            .rev()
            .position(|s| *s != ".")
            .map(|idx| res.len() - idx - 1)
            .expect("no empty ch in string from end");

        while i < j {
            match (res[i].as_str(), res[j].as_str()) {
                (".", ".") => {
                    j -= 1;
                }
                (".", _) => {
                    res[i] = res[j].clone();
                    res[j] = ".".to_string();
                    i += 1;
                    j -= 1;
                }
                (_, _) => {
                    i += 1;
                }
            }
        }

        res.iter()
            .filter(|ch| *ch != ".")
            .enumerate()
            .fold(0, |ctx, (i, ch)| {
                ctx + ch.to_string().parse::<i64>().unwrap() * i as i64
            })
    }

    fn part2(input: String) -> impl Display {
        ""
    }
}
