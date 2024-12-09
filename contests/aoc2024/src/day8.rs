use crate::*;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> HashMap<char, Vec<Point>> {
    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, line)| {
            line.char_indices()
                .filter(|&(_, c)| c != '.')
                .for_each(|(j, c)| {
                    map.entry(c).or_insert_with(Vec::new).push(Point {
                        x: j as i32,
                        y: i as i32,
                    })
                });
            map
        })
}

impl Day8 for Year2024 {
    fn part1(input: String) -> impl Display {
        let antennas = parse_input(&input);
        let (width, height) = get_dimensions(&input);

        antennas
            .iter()
            .fold(
                HashSet::new(),
                |mut antinode_locations: HashSet<Point>, (_, locations)| {
                    locations.iter().tuple_combinations().for_each(|(&a, &b)| {
                        let dist_vec = subtract_points(a, b);

                        let na = add_points(a, dist_vec);
                        if check_point_inbounds(na, width, height) {
                            antinode_locations.insert(na);
                        }

                        let nb = subtract_points(b, dist_vec);
                        if check_point_inbounds(nb, width, height) {
                            antinode_locations.insert(nb);
                        }
                    });

                    antinode_locations
                },
            )
            .len()
    }

    fn part2(input: String) -> impl Display {
        let antennas = parse_input(&input);
        let (width, height) = get_dimensions(&input);

        antennas
            .iter()
            .fold(
                HashSet::new(),
                |mut antinode_locations: HashSet<Point>, (_, locations)| {
                    locations.iter().tuple_combinations().for_each(|(&a, &b)| {
                        let dist_vec = subtract_points(a, b);

                        let mut na = add_points(a, dist_vec);
                        while check_point_inbounds(na, width, height) {
                            antinode_locations.insert(na);
                            na = add_points(na, dist_vec);
                        }

                        let mut nb = subtract_points(b, dist_vec);
                        while check_point_inbounds(nb, width, height) {
                            antinode_locations.insert(nb);
                            nb = subtract_points(nb, dist_vec);
                        }
                    });
                    if locations.len() > 1 {
                        antinode_locations.extend(locations.iter());
                    }

                    antinode_locations
                },
            )
            .len()
    }
}

fn get_dimensions(input: &str) -> (i32, i32) {
    let w = input.split_once('\n').unwrap().0.len();
    let h = input.lines().count();
    (w as i32, h as i32)
}

fn check_point_inbounds(p: Point, width: i32, height: i32) -> bool {
    0 <= p.x && p.x < width && 0 <= p.y && p.y < height
}

fn add_points(a: Point, b: Point) -> Point {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

fn subtract_points(a: Point, b: Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}
