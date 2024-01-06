use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_grouped_input;
use ya_advent_lib::grid::Grid;

fn mkgrid(inp: &[String]) -> Grid<char> {
    Grid::from_input(inp, '.', 0)
}

#[derive(Debug)]
enum Reflection {
    Horiz(usize),
    Vert(usize),
}
impl Reflection {
    fn value(&self) -> usize {
        match self {
            Reflection::Horiz(n) => 100 * (n+1),
            Reflection::Vert(n) => n+1,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum SmudgeEq {
    Equal,
    Smudged,
    NoMatch,
}

fn smudge_eq(a: &Vec<char>, b: &Vec<char>, check_for_smudge: bool) -> SmudgeEq {
    assert_eq!(a.len(), b.len());
    let m = a.iter().zip(b.iter())
        .filter(|(aa,bb)| *aa == *bb)
        .count();
    if m == a.len() {
        SmudgeEq::Equal
    }
    else if check_for_smudge && m == a.len() - 1 {
        SmudgeEq::Smudged
    }
    else {
        SmudgeEq::NoMatch
    }
}

fn check_symmetry(rows: &Vec<Vec<char>>, part2: bool) -> Option<usize> {
    for (smudge, candidate) in rows.iter().enumerate()
            .tuple_windows()
            .map(|((idx,r1),(_,r2))| (smudge_eq(r1, r2, part2), idx))
            .filter(|(sm, _)| *sm != SmudgeEq::NoMatch) {
        let mut good = true;
        let mut check_for_smudge = part2 && smudge == SmudgeEq::Equal;
        for i in 0..candidate {
            let a = candidate - 1 - i;
            let b = candidate + 2 + i;
            if b >= rows.len() {
                break;
            }
            match smudge_eq(&rows[a], &rows[b], check_for_smudge) {
                SmudgeEq::Equal => {},
                SmudgeEq::Smudged => {
                    if check_for_smudge {
                        check_for_smudge = false;
                    }
                    else {
                        good = false;
                        break;
                    }
                },
                SmudgeEq::NoMatch => {
                    good = false;
                    break;
                },
            }
        }
        if good && (!part2 || !check_for_smudge) {
            return Some(candidate);
        }
    }
    None
}

fn find_reflection(grid: &Grid<char>, part2: bool) -> Reflection {
    let rows = grid.rows().collect();
    if let Some(n) = check_symmetry(&rows, part2) {
        return Reflection::Horiz(n);
    }
    let rows = grid.cols().collect();
    if let Some(n) = check_symmetry(&rows, part2) {
        return Reflection::Vert(n);
    }
    grid.print();
    panic!();
}

fn part1(input: &[Vec<String>]) -> usize {
    input.iter()
        .map(|s| mkgrid(s))
        .map(|g| find_reflection(&g, false))
        .map(|r| r.value())
        .sum()
}

fn part2(input: &[Vec<String>]) -> usize {
    input.iter()
        .map(|s| mkgrid(s))
        .map(|g| find_reflection(&g, true))
        .map(|r| r.value())
        .sum()
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day13_test() {
        let input: Vec<Vec<String>> = grouped_test_input(include_str!("day13.testinput"));
        assert_eq!(part1(&input), 405);
        assert_eq!(part2(&input), 400);
    }
}
