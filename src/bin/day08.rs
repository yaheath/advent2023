use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use advent_lib::read::read_sectioned_input;

#[derive(Debug)]
struct Input {
    id: String,
    l: String,
    r: String,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\w+) = .(\w+), (\w+)"
            ).unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let id:String = caps.get(1).unwrap().as_str().into();
            let l:String = caps.get(2).unwrap().as_str().into();
            let r:String = caps.get(3).unwrap().as_str().into();
            Ok(Input {id, l, r})
        }
        else {
            Err(())
        }
    }
}

fn part1(input: &(Vec<String>, Vec<Input>)) -> usize {
    let map: HashMap<String, &Input> = input.1.iter().map(|i| (i.id.clone(), i)).collect();
    let mut steps = 0;
    let mut current = "AAA";
    let target = "ZZZ";
    let mut diriter = input.0[0].chars().cycle();
    while current != target {
        let cur = map[current];
        current = match diriter.next().unwrap() {
            'L' => &cur.l,
            'R' => &cur.r,
            _ => panic!(),
        };
        steps += 1;
    }
    steps
}

fn part2(input: &(Vec<String>, Vec<Input>)) -> usize {
    let map: HashMap<String, &Input> = input.1.iter().map(|i| (i.id.clone(), i)).collect();
    let mut steps = 0;
    let initial = input.1.iter().filter(|i| i.id.ends_with('A')).map(|i| &i.id).collect::<Vec<_>>();
    let mut currents = initial.clone();
    let mut diriter = input.0[0].chars().cycle();
    let mut cycles = vec![0; initial.len()];

    while cycles.iter().any(|n| *n == 0) {
        steps += 1;
        let dir = diriter.next().unwrap();
        for c in currents.iter_mut() {
            let cur = map.get(*c).unwrap();
            *c = match dir {
                'L' => &cur.l,
                'R' => &cur.r,
                _ => panic!(),
            };
        }
        for i in 0..currents.len() {
            if cycles[i] == 0 && currents[i].ends_with('Z') {
                cycles[i] = steps;
            }
        }
    }
    cycles.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = a;
        a = b;
        b = t % b;
    }
    a
}

fn main() {
    let input: (Vec<String>, Vec<Input>) = read_sectioned_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::sectioned_test_input;

    #[test]
    fn day08_test() {
        let input: (Vec<String>, Vec<Input>) = sectioned_test_input(
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
");
        assert_eq!(part1(&input), 2);
        let input: (Vec<String>, Vec<Input>) = sectioned_test_input(
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
");
        assert_eq!(part2(&input), 6);
    }
}
