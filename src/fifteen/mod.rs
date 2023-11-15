use crate::Runner;

pub struct Aoc2015;

impl Runner for Aoc2015 {
    fn new() -> Self {
        Self
    }

    fn run(&self, day: String, input: String) -> String {
        match day.as_str() {
            "1" => day_one(input),
            "2" => day_one_2(input),
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
}
