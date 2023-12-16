use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct RGB {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for RGB {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        s.split(", ").for_each(|ss| {
            let v: Vec<_> = ss.split(' ').collect();
            let n = v[0].parse::<usize>().unwrap();
            match v[1] {
                "red" => { red = n; },
                "blue" => { blue = n; },
                "green" => { green = n; },
                _ => panic!(),
            };
        });
        Ok(RGB { red, green, blue })
    }
}

struct Game {
    id: usize,
    sets: Vec<RGB>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g = s.split(": ").collect::<Vec<_>>();
        let id = g[0].split(' ').last().unwrap().parse::<usize>().unwrap();
        let sets = g[1].split("; ").map(|x| x.parse::<RGB>().unwrap()).collect();
        Ok(Game{id, sets})
    }
}

fn possible(game: &Game) -> bool {
    let r_thresh = 12;
    let g_thresh = 13;
    let b_thresh = 14;
    game.sets.iter()
        .all(|rgb| rgb.red <= r_thresh && rgb.green <= g_thresh && rgb.blue <= b_thresh)
}

fn part1(input: &Vec<Game>) -> usize {
    input.iter()
        .filter(|game| possible(game))
        .map(|game| game.id)
        .sum()
}

fn power(game: &Game) -> usize {
    let (r, g, b) = game.sets.iter()
        .fold((0,0,0), |acc, rgb| (
            acc.0.max(rgb.red),
            acc.1.max(rgb.green),
            acc.2.max(rgb.blue),
        ));
    r * g * b
}

fn part2(input: &Vec<Game>) -> usize {
    input.iter()
        .map(|game| power(game))
        .sum()
}

fn main() {
    let input: Vec<Game> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day02_test() {
        let input: Vec<Game> = test_input(include_str!("day02.testinput"));
        assert_eq!(part1(&input), 8);
        assert_eq!(part2(&input), 2286);
    }
}
