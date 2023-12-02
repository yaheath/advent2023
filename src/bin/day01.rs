use std::collections::HashMap;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;

trait FirstLastAdaptor: Iterator {
    fn first_last(self) -> Option<(Self::Item, Self::Item)>;
}
impl <I> FirstLastAdaptor for I
where I: Iterator, I::Item: Clone {
    fn first_last(mut self) -> Option<(I::Item, I::Item)>
    where I::Item: Clone {
        if let Some(first) = self.next() {
            let mut last = first.clone();
            while let Some(next) = self.next() {
                last = next;
            }
            Some((first, last))
        }
        else {
            None
        }
    }
}

fn part1(input: &Vec<String>) -> u64 {
    input.iter()
        .map(|s| {
            let digits = s.chars()
                .filter(|&c| c >= '1' && c <= '9')
                .first_last()
                .unwrap();
            (digits.0 as u64 - '0' as u64) * 10 + (digits.1 as u64 - '0' as u64)
        })
        .sum()
}

fn part2(input: &Vec<String>) -> u64 {
    let digits: HashMap<&str, u64> = HashMap::from_iter([
        ("1", 1), ("2", 2), ("3", 3), ("4", 4),
        ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
        ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ]);
    input.iter()
        .map(|s| {
            let d = digits.iter()
                .map(|(k, &v)| s.match_indices(k).map(move |(idx, _)| (idx, v)))
                .flatten()
                .sorted_unstable_by_key(|t| t.0)
                .map(|(_,v)| v)
                .first_last()
                .unwrap();
            d.0 * 10 + d.1
        })
        .sum()
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day01_test() {
        let input: Vec<String> = test_input(include_str!("day01.testinput"));
        assert_eq!(part1(&input), 142);
        let input: Vec<String> = test_input(include_str!("day01.testinput2"));
        assert_eq!(part2(&input), 281);
    }
}
