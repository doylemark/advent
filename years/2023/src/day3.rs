use std::fmt::Display;

use crate::*;

impl Day3 for Year2023 {
    fn part1(input: String) -> impl Display {
        let matrix = input
            .lines()
            .map(|ln| ln.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut nums: Vec<(i32, (usize, usize, usize))> = vec![];

        for i in 0..matrix.len() {
            let row = matrix.get(i).expect("infallible");

            let mut j = 0;
            while j < row.len() {
                let mut cur_num = String::new();
                let mut cur = row.get(j).unwrap();

                while cur.is_ascii_digit() || (cur_num.is_empty() && *cur == '-') {
                    cur_num.extend([cur]);

                    if let Some(next) = row.get(j + 1) {
                        cur = next;
                        j += 1;
                    }
                }
                if !cur_num.is_empty() {
                    nums.push((
                        cur_num.parse::<i32>().unwrap(),
                        (j - cur_num.len(), i, cur_num.len()),
                    ));
                }
                j += 1;
            }
        }

        let mut sum = 0;

        for (num, (mut x, y, len)) in nums {
            let mut i = 0;
            while i < len {
                let mut has_adjacent = false;

                aoc::vec::visit_offsets(&matrix, x, y, |item: &char| {
                    if !item.is_ascii_digit() && *item != '.' {
                        has_adjacent = true;
                    }
                });

                if has_adjacent {
                    sum += num;
                    break;
                }

                i += 1;
                x += 1;
            }
        }

        sum.to_string()
    }

    fn part2(input: String) -> impl Display {
        input
    }
}
