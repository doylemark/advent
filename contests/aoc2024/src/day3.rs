use crate::*;
use std::fmt::Display;

impl Day3 for Year2024 {
    fn part1(input: String) -> impl Display {
        let mut fsm = FSM::new();
        let mut sum = 0;

        for ch in input.replace("\n", "").chars() {
            match fsm.next(ch) {
                State::End(a, b) => sum += a * b,
                _ => (),
            };
        }

        sum
    }

    fn part2(input: String) -> impl Display {
        input
    }
}

#[derive(Debug)]
enum State {
    InMul(char),
    OpenParens(Option<String>),
    CloseParens(String, Option<String>),
    End(i32, i32),
    Idle,
}

struct FSM {
    inner: State,
}

impl FSM {
    fn new() -> FSM {
        FSM { inner: State::Idle }
    }

    fn state(&self) -> &State {
        &self.inner
    }

    fn transition(&mut self, new: State) {
        println!("new=[{new:?}]");
        self.inner = new;
    }

    fn next(&mut self, ch: char) -> &State {
        let new = match &self.inner {
            State::Idle | State::End(_, _) => {
                if ch == 'm' {
                    State::InMul('u')
                } else {
                    State::Idle
                }
            }
            State::InMul(next) => {
                if ch != *next {
                    State::Idle
                } else {
                    match next {
                        'u' => State::InMul('l'),
                        'l' => State::InMul('('),
                        '(' => State::OpenParens(None),
                        _ => panic!("moved into invalid state InMul({next})"),
                    }
                }
            }
            State::OpenParens(cur) => match cur {
                Some(cur) => match ch {
                    '0'..='9' => State::OpenParens(Some(format!("{cur}{ch}"))),
                    ',' => State::CloseParens(cur.to_owned(), None),
                    _ => State::Idle,
                },
                None if ch.is_ascii_digit() => State::OpenParens(Some(ch.to_string())),
                None => State::Idle,
            },
            State::CloseParens(first, cur) => match cur {
                Some(cur) => match ch {
                    '0'..='9' => State::CloseParens(first.to_owned(), Some(format!("{cur}{ch}"))),
                    ')' => State::End(first.parse().unwrap(), cur.parse().unwrap()),
                    _ => State::Idle,
                },
                None if ch.is_ascii_digit() => {
                    State::CloseParens(first.to_owned(), Some(ch.to_string()))
                }
                None => State::Idle,
            },
        };

        self.transition(new);
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn fsm_returns_to_idle_on_bad_content_inside_parens() {
        let mut fsm = FSM::new();

        for ch in "mul(BAD)".chars() {
            fsm.next(ch);
        }

        assert_matches!(fsm.state(), State::Idle)
    }

    #[test]
    fn fsm_parses_first_digit_inside_parens() {
        let mut fsm = FSM::new();

        for ch in "mul(1".chars() {
            fsm.next(ch);
        }

        assert_matches!(fsm.state(), State::OpenParens(Some(_)))
    }

    #[test]
    fn fsm_parses_multi_digit_num_in_first_pos() {
        let mut fsm = FSM::new();

        for ch in "mul(100".chars() {
            fsm.next(ch);
        }

        match fsm.state() {
            State::OpenParens(Some(v)) => assert_eq!("100", v),
            _ => panic!("infallible"),
        }
    }

    #[test]
    fn fsm_moves_into_closed_paren_state() {
        let mut fsm = FSM::new();

        for ch in "mul(100,".chars() {
            fsm.next(ch);
        }

        assert_matches!(fsm.state(), State::CloseParens(_, None))
    }

    #[test]
    fn fsm_parses_num_in_second_position() {
        let mut fsm = FSM::new();

        for ch in "mul(100,20".chars() {
            fsm.next(ch);
        }

        match fsm.state() {
            State::CloseParens(_, Some(v)) => assert_eq!("20", v),
            _ => panic!("infallible"),
        }
    }

    #[test]
    fn fsm_moves_to_end_state() {
        let mut fsm = FSM::new();

        for ch in "mul(32,64)".chars() {
            fsm.next(ch);
        }

        assert_matches!(fsm.state(), State::End(32, 64))
    }
}
