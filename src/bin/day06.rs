use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn part1(input: &[String]) -> usize {
    input[0].split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .zip(input[1].split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
        )
        .map(|(t, d)| (1..t).map(|tt| (t-tt)*tt).filter(|&tt| tt > d).count())
        .product()
}

fn part2(input: &[String]) -> usize {
    let t = input[0]
        .split(':').nth(1).unwrap()
        .chars().filter(|c| *c != ' ')
        .collect::<String>().parse::<u64>().unwrap();
    let d = input[1]
        .split(':').nth(1).unwrap()
        .chars().filter(|c| *c != ' ')
        .collect::<String>().parse::<u64>().unwrap();
    (1..t).map(|tt| (t-tt)*tt).filter(|&tt| tt > d).count()
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
    fn day06_test() {
        let input: Vec<String> = test_input(include_str!("day06.testinput"));
        assert_eq!(part1(&input), 288);
        assert_eq!(part2(&input), 71503);
    }
}
