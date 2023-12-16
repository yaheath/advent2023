use std::collections::{BTreeSet, HashSet};
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

struct Galaxies {
    galaxies: HashSet<(i64,i64)>,
    rows: BTreeSet<i64>,
    cols: BTreeSet<i64>,
}

impl Galaxies {
    fn from_input(input: &Vec<String>) -> Self {
        let galaxies = HashSet::from_iter(
            input.iter()
                .enumerate()
                .flat_map(|(y, s)| s.match_indices('#')
                    .map(move |(x,_)| (x as i64, y as i64))
                )
        );
        let rows = BTreeSet::from_iter(
            galaxies.iter()
            .map(|(_,y)| *y as i64)
        );
        let cols = BTreeSet::from_iter(
            galaxies.iter()
            .map(|(x,_)| *x as i64)
        );
        Self {galaxies, rows, cols}
    }
    fn md_between(&self, x1: i64, y1: i64, x2: i64, y2: i64, exp_fact: i64) -> i64 {
        let xd = (x2 - x1).abs();
        let xsp = xd - self.cols.range(x1.min(x2) .. x1.max(x2)).count() as i64;
        let yd = (y2 - y1).abs();
        let ysp = yd - self.rows.range(y1.min(y2) .. y1.max(y2)).count() as i64;
        xd + yd + (xsp + ysp) * (exp_fact - 1)
    }
}

fn solve(galaxies: &Galaxies, exp_fact: i64) -> i64 {
    galaxies.galaxies.iter()
        .tuple_combinations()
        .map(|(a,b)| galaxies.md_between(a.0, a.1, b.0, b.1, exp_fact))
        .sum()
}

fn part1(galaxies: &Galaxies) -> i64 {
    solve(galaxies, 2)
}

fn part2(galaxies: &Galaxies) -> i64 {
    solve(galaxies, 1_000_000)
}

fn main() {
    let input: Vec<String> = read_input();
    let galaxies = Galaxies::from_input(&input);
    println!("Part 1: {}", part1(&galaxies));
    println!("Part 2: {}", part2(&galaxies));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day11_test() {
        let input: Vec<String> = test_input(include_str!("day11.testinput"));
        let galaxies = Galaxies::from_input(&input);
        assert_eq!(part1(&galaxies), 374);
        let ex:i64 = solve(&galaxies, 10);
        assert_eq!(ex, 1030);
        let ex:i64 = solve(&galaxies, 100);
        assert_eq!(ex, 8410);
    }
}
