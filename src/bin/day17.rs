use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::algorithm::a_star;

#[derive(Copy, Clone)]
struct Cell {
    loss: u8,
}

impl From<char> for Cell {
    fn from(v: char) -> Self {
        Cell { loss: (v as u8) - b'0' }
    }
}

fn solve(input: &[String], part2: bool) -> usize {
    let grid: Grid<Cell> = Grid::from_input(input, Cell {loss: 0}, 0);
    let target = Coord2D::new(grid.x_bounds().end - 1, grid.y_bounds().end - 1);
    let start = Coord2D::new(-1,0);
    let (minsteps, maxsteps) = if part2 { (4, 10) } else { (1, 3) };

    a_star(
        (start, CDir::E),
        |c| c.0 == target,
        |(loc, dir)| {
            if loc == start {
                vec![
                    ((Coord2D::new(0,0), CDir::E), 0),
                    ((Coord2D::new(0,0), CDir::S), 0),
                ]
            }
            else {
                let mut loc = loc;
                let mut cost = 0;
                let mut neighs: Vec<((Coord2D, CDir), usize)> = Vec::new();
                for s in 1 ..= maxsteps {
                    loc += dir;
                    if !grid.contains_coord(loc) { break; }
                    cost += grid.get_c(loc).loss as usize;
                    if s < minsteps { continue; }
                    neighs.push(((loc, dir.left()), cost));
                    neighs.push(((loc, dir.right()), cost));
                }
                neighs
            }
        },
        |c| c.0.mdist_to(&target) as usize,
    ).unwrap()
}

fn part1(input: &[String]) -> usize {
    solve(input, false)
}

fn part2(input: &[String]) -> usize {
    solve(input, true)
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
    fn day17_test() {
        let input: Vec<String> = test_input(include_str!("day17.testinput"));
        assert_eq!(part1(&input), 102);
        assert_eq!(part2(&input), 94);

        let input: Vec<String> = test_input(
"111111111111
999999999991
999999999991
999999999991
999999999991
");
        assert_eq!(part2(&input), 71);
    }
}
