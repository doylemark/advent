use crate::*;
use std::fmt::Display;

impl Day9 for Year2024 {
    fn part1(input: String) -> impl Display {
        let res = input
            .trim()
            .chars()
            .map(|ch| ch.to_string())
            .collect::<Vec<String>>()
            .chunks(2)
            .enumerate()
            .flat_map(|(file_id, chunk)| {
                let size = chunk[0].parse::<usize>().unwrap();
                let empty = chunk
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_default();

                std::iter::repeat(file_id.to_string())
                    .take(size)
                    .chain(std::iter::repeat(".".to_string()).take(empty))
            })
            .collect::<Vec<_>>();

        let i = res
            .iter()
            .position(|s| *s == ".")
            .expect("no empty ch in string");

        let j = res
            .iter()
            .rev()
            .position(|s| *s != ".")
            .map(|idx| res.len() - idx - 1)
            .expect("no empty ch in string from end");

        (i..=j)
            .fold((res, i, j), |(mut res, i, j), _| {
                match (res[i].as_str(), res[j].as_str()) {
                    (".", ".") => (res, i, j - 1),
                    (".", _) => {
                        res.swap(i, j);
                        (res, i + 1, j - 1)
                    }
                    (_, _) => (res, i + 1, j),
                }
            })
            .0
            .iter()
            .filter(|s| *s != ".")
            .enumerate()
            .fold(0, |ctx, (i, s)| {
                ctx + s.to_string().parse::<i64>().unwrap() * i as i64
            })
    }

    fn part2(input: String) -> impl Display {
        let res = input
            .trim()
            .chars()
            .map(|ch| ch.to_string())
            .collect::<Vec<String>>()
            .chunks(2)
            .enumerate()
            .flat_map(|(file_id, chunk)| {
                let size = chunk[0].parse::<usize>().unwrap();
                let empty = chunk
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_default();

                std::iter::repeat(file_id.to_string())
                    .take(size)
                    .chain(std::iter::repeat(".".to_string()).take(empty))
            })
            .collect::<Vec<_>>();

        let file_sizes: Vec<(String, usize)> = input
            .trim()
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .enumerate()
            .map(|(id, chunk)| (id.to_string(), chunk[0].to_digit(10).unwrap() as usize))
            .collect();

        let result = file_sizes.iter().rev().fold(res, |mut arr, (id, size)| {
            if let Some(curr_pos) = arr.iter().position(|x| x == id) {
                let mut best_pos = curr_pos;
                let mut current_empty = 0;
                let mut potential_pos = None;

                for i in 0..curr_pos {
                    if arr[i] == "." {
                        if potential_pos.is_none() {
                            potential_pos = Some(i);
                        }
                        current_empty += 1;
                        if current_empty >= *size {
                            best_pos = potential_pos.unwrap();
                            break;
                        }
                    } else {
                        current_empty = 0;
                        potential_pos = None;
                    }
                }

                if best_pos < curr_pos {
                    for i in curr_pos..curr_pos + size {
                        arr[i] = ".".to_string();
                    }
                    for i in best_pos..best_pos + size {
                        arr[i] = id.clone();
                    }
                } else {
                }
            }
            arr
        });

        result
            .iter()
            .enumerate()
            .filter(|(_, s)| *s != ".")
            .map(|(i, s)| i as i64 * s.parse::<i64>().unwrap())
            .sum::<i64>()
    }
}
