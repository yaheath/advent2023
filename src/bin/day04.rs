use std::collections::HashSet;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct Input {
    winners: HashSet<u32>,
    have: HashSet<u32>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(": ").nth(1).unwrap();
        let mut itr = s.split(" | ");
        let winners = itr.next().unwrap().split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let have = itr.next().unwrap().split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        Ok(Input{winners, have})
    }
}

fn part1(input: &[Input]) -> u32 {
    input.iter()
        .map(|card| card.winners.intersection(&card.have).count())
        .map(|c| match c { 0 => 0, n => 2u32.pow(n as u32 - 1) })
        .sum()
}

fn part2(input: &[Input]) -> usize {
    let mut counts: Vec<usize> = vec![1; input.len()];

    input.iter()
        .enumerate()
        .for_each(|(idx, card)| {
            let w = card.winners.intersection(&card.have).count();
            for n in idx + 1 .. (idx + 1 + w).min(counts.len()) {
                counts[n] += counts[idx];
            }
        });
    counts.iter().sum()
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
    fn day04_test() {
        let input: Vec<Input> = test_input(include_str!("day04.testinput"));
        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 30);
    }
}
