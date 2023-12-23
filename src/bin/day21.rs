use std::collections::{HashSet, VecDeque};
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::coords::Coord2D;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Plot,
    Rock,
    Start
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Plot,
            '#' => Cell::Rock,
            'S' => Cell::Start,
            _ => panic!(),
        }
    }
}

pub fn mkgrid(input: &Vec<String>) -> Grid<Cell> {
    Grid::from_input(input, Cell::Plot, 0)
}

pub fn fill(grid: &Grid<Cell>, start: Coord2D, initial_step: usize, max_steps: usize) -> HashSet<Coord2D> {
    let mut out = HashSet::new();
    let mut queue:VecDeque<(Coord2D, usize)> = VecDeque::new();
    let mut stepped: HashSet<Coord2D> = HashSet::new();
    queue.push_back((start, initial_step));
    while let Some((pos, steps)) = queue.pop_front() {
        if steps < max_steps && steps % 2 == 0 || steps == max_steps {
            out.insert(pos);
            if steps == max_steps { continue; }
        }
        pos.neighbors4()
            .iter()
            .filter(|p| grid.contains_coord(**p) && grid.get_c(**p) != Cell::Rock)
            .for_each(|p| {
                if !stepped.contains(p) {
                    queue.push_back((*p, steps + 1));
                    stepped.insert(*p);
                }
            });
    }
    out
}

fn part1(input: &Vec<String>) -> usize {
    let grid = mkgrid(input);
    let (x, y) = grid.find(|c,_,_| c == Cell::Start).unwrap();
    fill(&grid, Coord2D::new(x, y), 0, 64).len()
}

fn bigfill(grid: &Grid<Cell>, start: Coord2D, maxsteps: usize) -> usize {
    0
}

fn part2(input: &Vec<String>) -> usize {
    let grid = mkgrid(input);
    let (x, y) = grid.find(|c,_,_| c == Cell::Start).unwrap();
    bigfill(&grid, Coord2D::new(x, y), 26501365)
}

#[allow(dead_code)]
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
    fn day21_test() {
        let input: Vec<String> = test_input(include_str!("day21.testinput"));
        let grid = mkgrid(&input);
        let (x, y) = grid.find(|c,_,_| c == Cell::Start).unwrap();
        assert_eq!(fill(&grid, Coord2D::new(x, y), 0, 6).len(), 16);
        assert_eq!(bigfill(&grid, Coord2D::new(x, y), 10), 50);
        assert_eq!(bigfill(&grid, Coord2D::new(x, y), 50), 1594);
        assert_eq!(bigfill(&grid, Coord2D::new(x, y), 1000), 668697);
        assert_eq!(bigfill(&grid, Coord2D::new(x, y), 5000), 16733044);
    }
}
