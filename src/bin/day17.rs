use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::coords::{CDir, Coord2D};

#[derive(Copy, Clone)]
struct Cell {
    loss: u8,
}

impl From<char> for Cell {
    fn from(v: char) -> Self {
        Cell { loss: (v as u8) - b'0' }
    }
}

fn solve(input: &Vec<String>, part2: bool) -> usize {
    let grid: Grid<Cell> = Grid::from_input(input, Cell {loss: 0}, 0);
    let target = Coord2D::new(grid.x_bounds().end - 1, grid.y_bounds().end - 1);
    let mut queue: BinaryHeap<(Reverse<usize>,Coord2D,usize,CDir)> = BinaryHeap::new();
    let mut traversed: HashMap<(Coord2D, CDir, usize), usize> = HashMap::new();

    let start = Coord2D::new(0,0);
    queue.push((Reverse(target.mdist_to(&start) as usize), start, 1, CDir::E));
    queue.push((Reverse(target.mdist_to(&start) as usize), start, 1, CDir::S));
    let minsteps = if part2 { 4 } else { 0 };
    let maxsteps = if part2 { 10 } else { 3 };

    while let Some((cost, loc, steps, dir)) = queue.pop() {
        if loc == target {
            if !part2 || steps >= 4 {
                return cost.0;
            }
            continue;
        }
        let mut nextdirs: Vec<(CDir, usize)> = Vec::new();
        if steps >= minsteps {
            nextdirs.push((dir.left(), 1));
            nextdirs.push((dir.right(), 1));
        }
        if steps < maxsteps {
            nextdirs.push((dir, steps + 1));
        }
        for (d, nsteps) in nextdirs {
            let nloc = loc + d;
            if !grid.contains_coord(nloc) { continue; }
            let ncost = grid.get(nloc.x, nloc.y).loss as usize + (
                cost.0 - loc.mdist_to(&target) as usize
                );
            let tkey = (nloc, d, nsteps);
            if !traversed.contains_key(&tkey) || traversed[&tkey] > ncost {
                traversed.insert(tkey, ncost);
                queue.push((Reverse(ncost + nloc.mdist_to(&target) as usize), nloc, nsteps, d));
            }
        }
    }
    panic!();
}

fn part1(input: &Vec<String>) -> usize {
    solve(input, false)
}

fn part2(input: &Vec<String>) -> usize {
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
