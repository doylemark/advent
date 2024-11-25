use y2024::run as run2024;

mod cmd;

fn main() {
    let (year, day, part) = cmd::parse_cmd(
        std::env::args()
            .collect::<Vec<_>>()
            .get(1)
            .expect("no command"),
    );

    let input = std::fs::read_to_string("./input.txt").expect("failed to read input");
    let expected_output =
        std::fs::read_to_string("./expected_output.txt").expect("failed to read expected output");

    let output = match year {
        24 => run2024(day, part, input),
        _ => panic!("year implemented"),
    };

    println!("{output}");
    assert_eq!(output, expected_output);
}
