use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::read::read_input;

struct Input {
    dir: CDir,
    steps: i64,
    color: u32,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\w) (\d+) .#(\w+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let dir = match caps.get(1).unwrap().as_str().chars().next().unwrap() {
                'U' => CDir::N,
                'D' => CDir::S,
                'L' => CDir::W,
                'R' => CDir::E,
                _ => panic!(),
            };
            let steps:i64 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let color:u32 = u32::from_str_radix(caps.get(3).unwrap().as_str(), 16).unwrap();
            Ok(Input {dir, steps, color})
        }
        else {
            Err(())
        }
    }
}

fn polygon_from_input(input: &Vec<Input>) -> Vec<Coord2D> {
    let mut poly = Vec::new();
    let mut pos = Coord2D::new(0, 0);
    for i in input {
        pos += Into::<Coord2D>::into(i.dir) * i.steps;
        poly.push(pos.clone());
    }
    assert_eq!(poly[poly.len() - 1], Coord2D::new(0,0));
    poly
}

fn polygon_from_input_2(input: &Vec<Input>) -> Vec<Coord2D> {
    let mut poly = Vec::new();
    let mut pos = Coord2D::new(0, 0);
    for i in input {
        let dx = (i.color >> 4) as i64;
        let dir = match i.color & 0xf {
            0 => CDir::E,
            1 => CDir::S,
            2 => CDir::W,
            3 => CDir::N,
            _ => panic!(),
        };
        pos += Into::<Coord2D>::into(dir) * dx;
        poly.push(pos.clone());
    }
    assert_eq!(poly[poly.len() - 1], Coord2D::new(0,0));
    poly
}

fn poly_area(poly: &Vec<Coord2D>) -> i64 {
    let (sum1, sum2) = poly.iter()
        .circular_tuple_windows()
        .map(|(a, b)| (a.x * b.y, a.y * b.x))
        .fold((0, 0), |(sum1, sum2), (a, b)| (sum1 + a, sum2 + b));
    let intarea = (sum1 - sum2).abs() / 2;
    let edgearea = poly.iter()
        .circular_tuple_windows()
        .map(|(a, b)| (a.x - b.x).abs() + (a.y - b.y).abs())
        .sum::<i64>() / 2 + 1;
    intarea + edgearea
}

fn part1(input: &Vec<Input>) -> i64 {
    let polygon = polygon_from_input(&input);
    poly_area(&polygon)
}

fn part2(input: &Vec<Input>) -> i64 {
    let polygon = polygon_from_input_2(&input);
    poly_area(&polygon)
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day18_test() {
        let input: Vec<Input> = test_input(include_str!("day18.testinput"));
        assert_eq!(part1(&input), 62);
        assert_eq!(part2(&input), 952408144115);
    }
}
