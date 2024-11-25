use std::iter::from_fn;

#[derive(Debug)]
pub struct Cmd {
    pub year: i8,
    pub day: i8,
    pub part: i8,
}

#[derive(Debug)]
pub enum Token {
    Year,
    Day,
    Part,
    Number(i8),
}

pub fn parse_cmd(arg: &String) -> (i8, i8, i8) {
    let mut iter = arg.trim().chars().peekable();
    let mut tokens: Vec<Token> = vec![];

    while let Some(char) = iter.next() {
        tokens.push(match char {
            'd' => Token::Day,
            'p' => Token::Part,
            'y' => Token::Year,
            '1'..='9' => {
                let n: i8 = std::iter::once(char)
                    .chain(from_fn(|| iter.by_ref().next_if(|ch| ch.is_ascii_digit())))
                    .collect::<String>()
                    .parse()
                    .expect("failed to parse digits");

                Token::Number(n)
            }
            _ => panic!("parse error: expected d<int>p<int>"),
        });
    }

    let mut toks = tokens.iter().peekable();
    let mut cmd = Cmd {
        day: 1,
        part: 1,
        year: 24,
    };

    while let Some(tok) = toks.next() {
        match tok {
            Token::Day => {
                if let Some(Token::Number(n)) = toks.peek() {
                    cmd.day = n.to_owned();
                    toks.next();
                }
            }
            Token::Part => {
                if let Some(Token::Number(n)) = toks.peek() {
                    cmd.part = n.to_owned();
                    toks.next();
                }
            }
            Token::Year => {
                if let Some(Token::Number(n)) = toks.peek() {
                    cmd.year = n.to_owned();
                    toks.next();
                }
            }
            Token::Number(_) => panic!("parse error: expect d or p"),
        }
    }

    (cmd.year, cmd.day, cmd.part)
}
