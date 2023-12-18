use std::collections::{HashSet, VecDeque};
use std::iter;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

/*
enum Cell {
    Empty,
    HSplit,
    VSplit,
    FwdSlash,
    BckSlash
}
*/

fn calc_energized(grid: &Grid<char>, initial_loc: Coord2D, initial_dir: CDir) -> usize {
    let mut energized: HashSet<Coord2D> = HashSet::new();
    let mut queue: VecDeque<(CDir, Coord2D)> = VecDeque::new();
    queue.push_back((initial_dir, initial_loc));
    let mut traversed: HashSet<(CDir, Coord2D)> = HashSet:: new();
    while let Some((dir, loc)) = queue.pop_front() {
        if !grid.x_bounds().contains(&loc.x) || !grid.y_bounds().contains(&loc.y) {
            continue;
        }
        if traversed.contains(&(dir, loc)) {
            continue;
        }
        traversed.insert((dir, loc));
        energized.insert(loc);
        match (grid.get(loc.x, loc.y), dir) {
            ('|', CDir::W) |
            ('|', CDir::E) => {
                queue.push_back((CDir::N, loc + CDir::N));
                queue.push_back((CDir::S, loc + CDir::S));
            },
            ('-', CDir::N) |
            ('-', CDir::S) => {
                queue.push_back((CDir::E, loc + CDir::E));
                queue.push_back((CDir::W, loc + CDir::W));
            },
            ('/', CDir::N) => { queue.push_back((CDir::E, loc + CDir::E)); },
            ('/', CDir::S) => { queue.push_back((CDir::W, loc + CDir::W)); },
            ('/', CDir::E) => { queue.push_back((CDir::N, loc + CDir::N)); },
            ('/', CDir::W) => { queue.push_back((CDir::S, loc + CDir::S)); },
            ('\\', CDir::N) => { queue.push_back((CDir::W, loc + CDir::W)); },
            ('\\', CDir::S) => { queue.push_back((CDir::E, loc + CDir::E)); },
            ('\\', CDir::E) => { queue.push_back((CDir::S, loc + CDir::S)); },
            ('\\', CDir::W) => { queue.push_back((CDir::N, loc + CDir::N)); },

            (_, _) => { queue.push_back((dir, loc + dir)); },
        }
    }
    energized.len()
}

fn part1(input: &Vec<String>) -> usize {
    let grid: Grid<char> = Grid::from_input(input, '.', 0);
    calc_energized(&grid, Coord2D::new(0,0), CDir::E)
}

fn part2(input: &Vec<String>) -> usize {
    let grid: Grid<char> = Grid::from_input(input, '.', 0);
    [CDir::E, CDir::W, CDir::S, CDir::N]
        .into_iter()
        .flat_map(|dir| {
            let coords: Vec<(i64,i64)> = match dir {
                CDir::E => Vec::from_iter(
                    iter::once(0).cartesian_product(grid.y_bounds())
                ),
                CDir::W => Vec::from_iter(
                    iter::once(grid.x_bounds().end - 1).cartesian_product(grid.y_bounds())
                ),
                CDir::S => Vec::from_iter(
                    grid.x_bounds().cartesian_product(iter::once(0))
                ),
                CDir::N => Vec::from_iter(
                    grid.x_bounds().cartesian_product(iter::once(grid.y_bounds().end-1))
                ),
            };
            Vec::from_iter(
                coords.iter()
                .map(|(x, y)| calc_energized(&grid, Coord2D::new(*x,*y), dir))
            )
        })
        .max()
        .unwrap()
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
    fn day16_test() {
        let input: Vec<String> = test_input(include_str!("day16.testinput"));
        assert_eq!(part1(&input), 46);
        assert_eq!(part2(&input), 51);
    }
}
