use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;
use ya_advent_lib::iter::FirstLast;

fn part1(input: &Vec<String>) -> u64 {
    input.iter()
        .map(|s| {
            s.chars()
                .filter(|&c| c >= '1' && c <= '9')
                .first_last()
                .map(|(f,l)| (f as u64 - '0' as u64) * 10 + (l as u64 - '0' as u64))
                .unwrap()
        })
        .sum()
}

fn part2(input: &Vec<String>) -> u64 {
    input.iter()
        .map(|s| {
            [
                ("1", 1), ("2", 2), ("3", 3), ("4", 4),
                ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
                ("one", 1), ("two", 2), ("three", 3), ("four", 4),
                ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
            ].iter()
                .map(|(k, v)| s.match_indices(k).map(move |(idx, _)| (idx, v)))
                .flatten()
                .sorted_unstable_by_key(|t| t.0)
                .map(|(_,v)| v)
                .first_last()
                .map(|(f,l)| f * 10 + l)
                .unwrap()
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
    use ya_advent_lib::read::test_input;

    #[test]
    fn day01_test() {
        assert_eq!(
            part1(&test_input(include_str!("day01.testinput"))),
            142
        );
        assert_eq!(
            part2(&test_input(include_str!("day01.testinput2"))),
            281
        );
    }
}
