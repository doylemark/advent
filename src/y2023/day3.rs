fn has_adj_symbol(x: i32, y: i32, matrix: &Vec<Vec<char>>) -> bool {
    let adjacent = vec![
        [0, 0],
        [1, 0],
        [2, 0],
        [0, 1],
        [2, 1],
        [0, 2],
        [1, 2],
        [2, 2],
    ];

    for [mut a, mut b] in adjacent {
        a -= 1;
        b -= 1;

        if let Some(row) = matrix.get((y + a) as usize) {
            if let Some(ch) = row.get((x + b) as usize) {
                if !ch.is_numeric() && *ch != '.' {
                    return true;
                }
            }
        }
    }

    false
}

pub fn day_3(input: String) -> i32 {
    let m: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|ln| ln.chars().collect())
        .collect();

    let mut nums: Vec<i32> = vec![];

    let mut current_num: Vec<char> = vec![];
    let mut start_idx: Option<usize> = None;

    for (i, row) in m.iter().enumerate() {
        'r: for (j, ch) in row.iter().enumerate() {
            if ch.is_numeric() && j != row.len() - 1 {
                if current_num.is_empty() {
                    start_idx = Some(j);
                }

                current_num.push(*ch);
            } else {
                if ch.is_numeric() && j == row.len() - 1 {
                    if current_num.is_empty() {
                        start_idx = Some(j);
                    }

                    current_num.push(*ch);
                }

                if current_num.is_empty() {
                    continue 'r;
                }

                let n = current_num
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .expect("invalid integer");

                if let Some(start) = start_idx {
                    for x in start..current_num.len() + start {
                        if has_adj_symbol(x as i32, i as i32, &m) {
                            nums.push(n);
                            break;
                        }
                    }

                    start_idx = None;
                    current_num.clear();
                } else {
                    panic!("Found num without start index")
                }
            }
        }
    }

    nums.iter().sum()
}

#[cfg(test)]
mod day_3_test {
    use crate::y2023::day3::has_adj_symbol;

    #[test]
    fn day_3_matches_diagonal() {
        let m = vec![
            vec!['.', '.', '*'],
            vec!['.', 'a', '.'],
            vec!['.', '.', '.'],
        ];
        assert!(has_adj_symbol(1, 1, &m))
    }

    #[test]
    fn day_3_matches_eol() {
        let m = vec![
            vec!['7', '8', '.', '.', '.', '.', '.', '.', '.', '.', '.', '9'],
            vec!['.', '5', '.', '.', '.', '.', '.', '2', '3', '.', '.', '$'],
            vec!['8', '.', '.', '.', '9', '0', '*', '1', '2', '.', '.', '.'],
        ];

        assert!(has_adj_symbol(0, 11, &m))
    }
}
