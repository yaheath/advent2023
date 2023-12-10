use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_input;

struct ValSeq {
    vals: Vec<i64>,
}

impl FromStr for ValSeq {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split(' ').map(|n| n.parse::<i64>().unwrap()).collect();
        Ok(ValSeq{vals})
    }
}

impl ValSeq {
    fn next_prev_val(&self) -> (i64, i64) {
        let vals: Vec<i64> = self.vals.iter()
            .tuple_windows()
            .map(|(a,b)| b - a)
            .collect();
        if vals.iter().all(|v| *v == 0) {
            return (
                self.vals[0],
                self.vals[self.vals.len() - 1],
            );
        }
        let (pv, nv) = ValSeq { vals }.next_prev_val();
        (
            self.vals[0] - pv,
            nv + self.vals[self.vals.len() - 1],
        )
    }
}

fn bothparts(input: &Vec<ValSeq>) -> (i64, i64) {
    input.iter()
        .map(|i| i.next_prev_val())
        .reduce(|a,b| (a.0+b.0, a.1+b.1))
        .unwrap()
}

fn main() {
    let input: Vec<ValSeq> = read_input();
    let (part2, part1) = bothparts(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day09_test() {
        let input: Vec<ValSeq> = test_input(include_str!("day09.testinput"));
        let (part2, part1) = bothparts(&input);
        assert_eq!(part1, 114);
        assert_eq!(part2, 2);
    }
}
