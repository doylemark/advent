#[derive(Copy, Clone)]
pub struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl Color {
    pub fn new(s: String) -> Color {
        let mut current_n = "".to_string();
        let mut c = Color { r: 0, g: 0, b: 0 };

        for ch in s.chars() {
            if ch.is_digit(10) {
                current_n += ch.to_string().as_str();
            }

            if ch.is_alphabetic() {
                if current_n.is_empty() {
                    continue;
                }

                let n = current_n.parse::<i32>().unwrap();
                current_n = "".to_string();

                match ch {
                    'r' => c.r += n,
                    'g' => c.g += n,
                    'b' => c.b += n,
                    _ => panic!("unknown color"),
                }
            }
        }

        c
    }

    pub fn power(&self) -> i32 {
        self.r * self.g * self.b
    }
}

pub fn day_2(input: String) -> i32 {
    input
        .split("\n")
        .filter_map(|ln| {
            let parts = ln.split(":").collect::<Vec<&str>>();

            let game_id = parts
                .first()
                .expect("missing game id")
                .trim_start_matches("Game ")
                .parse::<i32>()
                .expect("invalid game id");

            let handfuls = parts.last().expect("missing handfuls").split(";");

            if handfuls
                .clone()
                .map(|hf| Color::new(hf.to_string()))
                .all(|c| c.r <= 12 && c.g <= 13 && c.b <= 14)
            {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

pub fn day_2_2(input: String) -> i32 {
    input
        .split("\n")
        .filter_map(|ln| {
            let parts = ln.split(":").collect::<Vec<&str>>();

            let handfuls = parts.last().expect("missing handfuls").split(";");

            if let Some(mins) = handfuls
                .clone()
                .map(|hf| Color::new(hf.to_string()))
                .reduce(|mut c, cur| {
                    if cur.r > c.r {
                        c.r = cur.r
                    }

                    if cur.g > c.g {
                        c.g = cur.g
                    }

                    if cur.b > c.b {
                        c.b = cur.b
                    }

                    c
                })
            {
                Some(mins)
            } else {
                None
            }
        })
        .map(|c| c.power())
        .sum()
}
