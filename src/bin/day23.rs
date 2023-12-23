use std::collections::HashSet;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::coords::{Coord2D, CDir};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Wall,
    Open,
    Slope(CDir),
}
impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Open,
            '>' => Cell::Slope(CDir::E),
            '<' => Cell::Slope(CDir::W),
            '^' => Cell::Slope(CDir::N),
            'v' => Cell::Slope(CDir::S),
            _ => panic!(),
        }
    }
}

fn dir_from(a: Coord2D, b: Coord2D) -> CDir {
    for d in [CDir::N, CDir::E, CDir::S, CDir::W] {
        if a + d == b { return d; }
    }
    panic!();
}

fn search(grid: &Grid<Cell>, traversed: HashSet<Coord2D>, loc: Coord2D, target: Coord2D, part2: bool) -> usize {
    // follow the path starting at loc until we:
    //   hit a dead end (return 0)
    //   hit a fork (recurse down each path, return the recursion with most steps)
    //   hit the target (return steps traversed)
    let mut loc = loc;
    let mut traversed = traversed;
    let mut steps = 0;
    while loc != target  {
        steps += 1;
        let nei = loc.neighbors4().iter()
            .filter(|c| grid.contains_coord(**c) && !traversed.contains(c))
            .filter(|c| match grid.get_c(**c) {
                Cell::Open => true,
                Cell::Slope(CDir::N) if part2 || dir_from(loc, **c) != CDir::S => true,
                Cell::Slope(CDir::S) if part2 || dir_from(loc, **c) != CDir::N => true,
                Cell::Slope(CDir::E) if part2 || dir_from(loc, **c) != CDir::W => true,
                Cell::Slope(CDir::W) if part2 || dir_from(loc, **c) != CDir::E => true,
                _ => false,
            })
            .copied()
            .collect::<Vec<_>>();
        if nei.len() == 0 {
            return 0;
        }
        if nei.len() == 1 {
            loc = nei[0];
            traversed.insert(loc);
        }
        else {
            let next = nei.iter()
                .map(|n| {
                    let mut t = traversed.clone();
                    t.insert(*n);
                    search(&grid, t, *n, target, part2)
                })
                .max()
                .unwrap();
            return next + steps;
        }
    }
    steps
}

fn run(input: &Vec<String>, part2: bool) -> usize {
    let grid = Grid::from_input(input, Cell::Wall, 0);
    let mut traversed: HashSet<Coord2D> = HashSet::new();
    let start = Coord2D::new(1, 0);
    let end = Coord2D::new(grid.x_bounds().end - 2, grid.y_bounds().end - 1);
    traversed.insert(start);
    search(&grid, traversed, start, end, part2)
}

fn part1(input: &Vec<String>) -> usize {
    run(input, false)
}
fn part2(input: &Vec<String>) -> usize {
    run(input, true)
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
    fn day23_test() {
        let input: Vec<String> = test_input(include_str!("day23.testinput"));
        assert_eq!(part1(&input), 94);
        assert_eq!(part2(&input), 154);
    }
}
