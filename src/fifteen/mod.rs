use crate::Runner;

pub struct Aoc2015;

impl Runner for Aoc2015 {
    fn new() -> Self {
        Self
    }

    fn run(&self, day: String, input: String) -> String {
        match day.as_str() {
            "1" => day_one(input),
            "1.1" => day_one_2(input),
            "2" => day_two(input),
            "2.2" => day_two_2(input),
            _ => panic!("unknown day!"),
        }
    }
}

fn day_one(input: String) -> String {
    let mut open_count = 0;
    let mut close_count = 0;

    for char in input.chars() {
        if char == '(' {
            open_count += 1;
        } else {
            close_count += 1;
        }
    }

    (open_count - close_count).to_string()
}

fn day_one_2(input: String) -> String {
    let mut open_count = 0;
    let mut close_count = 0;

    for (i, char) in input.chars().enumerate() {
        if char == '(' {
            open_count += 1;
        } else {
            close_count += 1;
        }

        if open_count - close_count == -1 {
            return (i + 1).to_string();
        }
    }

    (open_count - close_count).to_string()
}

fn day_two(input: String) -> String {
    let mut total = 0;

    for line in input.lines() {
        let dims: Vec<i32> = line
            .split("x")
            .map(|n| n.trim().parse().expect("invalid dimension"))
            .collect();

        let l = dims.get(0).expect("Missing len");
        let w = dims.get(1).expect("Missing width");
        let h = dims.get(2).expect("Missing h");

        let top_size = l * w;
        let side_size = w * h;
        let front_back_size = h * l;

        let size = 2 * top_size + 2 * side_size + 2 * front_back_size;

        let excess = top_size.min(side_size).min(front_back_size);

        total += size + excess;
    }

    total.to_string()
}

fn day_two_2(input: String) -> String {
    let mut total = 0;

    for line in input.lines() {
        let dims: Vec<i32> = line
            .split("x")
            .map(|n| n.trim().parse().expect("invalid dimension"))
            .collect();

        let l = dims.get(0).expect("Missing len");
        let w = dims.get(1).expect("Missing width");
        let h = dims.get(2).expect("Missing h");

        let top_perim = l * 2 + w * 2;
        let side_perim = w * 2 + h * 2;
        let front_back_permit = h * 2 + l * 2;

        let min_perim = top_perim.min(side_perim).min(front_back_permit);

        let vol = l * w * h;

        total += vol + min_perim;
    }

    total.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_one_base_case() {
        assert_eq!(day_one("()()".to_string()), "0".to_string());
    }

    #[test]
    fn day_one_level_negative_3() {
        assert_eq!(day_one(")())())".to_string()), "-3".to_string());
        assert_eq!(day_one(")))".to_string()), "-3".to_string());
    }

    #[test]
    fn day_one_level_3() {
        assert_eq!(day_one("))(((((".to_string()), "3".to_string());
        assert_eq!(day_one("(()(()(".to_string()), "3".to_string());
    }

    #[test]
    fn day_one_2_base_case() {
        assert_eq!(day_one_2(")".to_string()), "1".to_string());
        assert_eq!(day_one_2("()())".to_string()), "5".to_string());
    }

    #[test]
    fn day_two_base_case() {
        assert_eq!(day_two("2x3x4".to_string()), "58".to_string());
    }

    #[test]
    fn day_two_2_base_case() {
        assert_eq!(day_two_2("2x3x4".to_string()), "34".to_string());
    }
}
