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
    let mut queue: BinaryHeap<(Reverse<usize>,Coord2D,CDir)> = BinaryHeap::new();
    let mut traversed: HashMap<(Coord2D, CDir), usize> = HashMap::new();

    let (minsteps, maxsteps) = if part2 { (4, 10) } else { (1, 3) };

    let start = Coord2D::new(0,0);

    queue.push((Reverse(start.mdist_to(&target) as usize), start, CDir::E));
    queue.push((Reverse(start.mdist_to(&target) as usize), start, CDir::S));

    while let Some((cost, loc, dir)) = queue.pop() {
        if loc == target {
            return cost.0;
        }
        let mut loc = loc;
        let mut cost = cost.0 - loc.mdist_to(&target) as usize;
        for s in 1 ..= maxsteps {
            loc += dir;
            if !grid.contains_coord(loc) { break; }
            cost += grid.get_c(loc).loss as usize;
            if s < minsteps { continue; }
            let qcost = Reverse(cost + loc.mdist_to(&target) as usize);
            if !traversed.contains_key(&(loc, dir)) || traversed[&(loc, dir)] > cost {
                traversed.insert((loc, dir), cost);
                queue.push((qcost, loc, dir.left()));
                queue.push((qcost, loc, dir.right()));
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
