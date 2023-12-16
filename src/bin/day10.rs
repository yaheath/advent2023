use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Ground,
    Start,
    PipeNS,
    PipeEW,
    PipeNE,
    PipeNW,
    PipeSE,
    PipeSW,
}

struct PipeMap {
    grid: Grid<Cell>,
    //start_pos: Coord2D,
    path: HashMap<Coord2D, usize>,
}

impl PipeMap {
    fn from_input(input: &Vec<String>) -> Self {
        let mut grid = Grid::from_input(input, Cell::Ground, 1, |c| match c {
            '|' => Cell::PipeNS,
            '-' => Cell::PipeEW,
            'L' => Cell::PipeNE,
            'J' => Cell::PipeNW,
            'F' => Cell::PipeSE,
            '7' => Cell::PipeSW,
            'S' => Cell::Start,
            '.' => Cell::Ground,
            _ => panic!(),
        });
        let start_pos = grid.iter_with_coord()
            .find(|(c,_,_)| *c == Cell::Start)
            .map(|(_,x,y)| Coord2D::new(x,y))
            .unwrap();

        let conn_n = match grid.get_c(start_pos + Coord2D::new(0, -1)) {
            Cell::PipeNS|Cell::PipeSE|Cell::PipeSW => true,
            _ => false,
        };
        let conn_s = match grid.get_c(start_pos + Coord2D::new(0, 1)) {
            Cell::PipeNS|Cell::PipeNE|Cell::PipeNW => true,
            _ => false,
        };
        let conn_e = match grid.get_c(start_pos + Coord2D::new(1, 0)) {
            Cell::PipeEW|Cell::PipeNW|Cell::PipeSW => true,
            _ => false,
        };
        let conn_w = match grid.get_c(start_pos + Coord2D::new(-1, 0)) {
            Cell::PipeEW|Cell::PipeNE|Cell::PipeSE => true,
            _ => false,
        };
        let replace = match (conn_n, conn_s, conn_e, conn_w) {
            (true, true, false, false) => Cell::PipeNS,
            (false, false, true, true) => Cell::PipeEW,
            (true, false, true, false) => Cell::PipeNE,
            (true, false, false, true) => Cell::PipeNW,
            (false, true, true, false) => Cell::PipeSE,
            (false, true, false, true) => Cell::PipeSW,
            _ => panic!("{conn_n} {conn_s} {conn_e} {conn_w}"),
        };
        grid.set_c(start_pos, replace);
        let path = find_path(&grid, start_pos);
        Self { grid, path }
    }

    fn find_interior(&self) -> HashSet<Coord2D> {
        let mut interior: HashSet<Coord2D> = HashSet::new();
        for y in self.grid.y_bounds() {
            let mut inside = false;
            for x in self.grid.x_bounds() {
                let loc = Coord2D::new(x, y);
                if self.path.contains_key(&loc) {
                    match self.grid.get_c(loc) {
                        Cell::PipeNS | Cell::PipeNE | Cell::PipeNW => { inside = !inside; },
                        _ => {},
                    }
                }
                else {
                    if inside {
                        interior.insert(loc);
                    }
                }
            }
        }
        interior
    }
}

fn find_path(grid: &Grid<Cell>, start_pos: Coord2D) -> HashMap<Coord2D, usize> {
    let mut path: HashMap<Coord2D,usize> = HashMap::new();
    let mut queue: VecDeque<Coord2D> = VecDeque::new();
    queue.push_back(start_pos);
    path.insert(start_pos, 0);
    while let Some(c) = queue.pop_front() {
        let n = path[&c] + 1;
        let nexts = match grid.get_c(c) {
            Cell::PipeNS => vec![Coord2D::new(0, -1), Coord2D::new(0, 1)],
            Cell::PipeEW => vec![Coord2D::new(1, 0), Coord2D::new(-1, 0)],
            Cell::PipeNE => vec![Coord2D::new(0, -1), Coord2D::new(1, 0)],
            Cell::PipeNW => vec![Coord2D::new(0, -1), Coord2D::new(-1, 0)],
            Cell::PipeSE => vec![Coord2D::new(0, 1), Coord2D::new(1, 0)],
            Cell::PipeSW => vec![Coord2D::new(0, 1), Coord2D::new(-1, 0)],
            _ => panic!(),
        };
        for next in nexts {
            let nextc = c + next;
            if !path.contains_key(&nextc) || path[&nextc] > n {
                path.insert(nextc, n);
                queue.push_back(nextc);
            }
        }
    }
    path
}

fn part1(input: &Vec<String>) -> usize {
    let map = PipeMap::from_input(input);
    map.path.into_values().max().unwrap()
}

fn part2(input: &Vec<String>) -> usize {
    let map = PipeMap::from_input(input);
    let interior = map.find_interior();
    interior.len()
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
    fn day10_test() {
        let input: Vec<String> = test_input(
"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
");
        assert_eq!(part1(&input), 4);
        let input: Vec<String> = test_input(
"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
");
        assert_eq!(part1(&input), 8);

        let input: Vec<String> = test_input(
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
");
        assert_eq!(part2(&input), 4);
        let input: Vec<String> = test_input(
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
");
        assert_eq!(part2(&input), 8);
        let input: Vec<String> = test_input(
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
");
        assert_eq!(part2(&input), 10);
    }
}
