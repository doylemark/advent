use std::collections::HashSet;

pub fn day_four(input: String) -> i32 {
    input
        .split("\n")
        .map(|ln| {
            let cards = ln
                .split(":")
                .last()
                .unwrap()
                .split("|")
                .map(|c| c.split(" ").filter(|s| *s != "").collect::<HashSet<&str>>())
                .collect::<Vec<HashSet<&str>>>();

            cards
                .first()
                .unwrap()
                .iter()
                .filter_map(|n| cards.last().unwrap().get(n))
                .fold(0, |ctx, _| if ctx == 0 { ctx + 1 } else { ctx * 2 })
        })
        .filter(|n| *n != 0)
        .sum()
}
