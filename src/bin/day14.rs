use std::collections::HashMap;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::{Grid, GridTransform};
use ya_advent_lib::coords::CDir;

#[derive(Copy,Clone,Eq,PartialEq)]
enum Cell {
    Round,
    Cube,
    Empty,
}
impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Empty,
            'O' => Cell::Round,
            '#' => Cell::Cube,
            _ => panic!(),
        }
    }
}
impl From<Cell> for char {
    fn from(value: Cell) -> char {
        match value {
            Cell::Empty => '.',
            Cell::Round => 'O',
            Cell::Cube  => '#',
        }
    }
}

fn mkgrid(input: &[String]) -> Grid<Cell> {
    Grid::from_input(input, Cell::Empty, 0)
}

fn move_rock(x: i64, y: i64, xform: GridTransform, grid: &mut Grid<Cell>) {
    let mut y = y;
    while y > 0 {
        match grid.get_xform(x, y - 1, xform) {
            Cell::Empty => {
                grid.set_xform(x, y, Cell::Empty, xform);
                grid.set_xform(x, y - 1, Cell::Round, xform);
                y -= 1;
            },
            _ => { return; }
        }
    }
}

fn tilt(grid: &mut Grid<Cell>, dir: CDir) {
    let xform = match dir {
        CDir::N => GridTransform::Identity,
        CDir::W => GridTransform::Rot90,
        CDir::S => GridTransform::Rot180,
        CDir::E => GridTransform::Rot270,
    };
    let xb = grid.x_bounds_xform(xform);
    let yb = grid.y_bounds_xform(xform);
    for y in yb {
        for x in xb.clone() {
            if grid.get_xform(x, y, xform) == Cell::Round {
                move_rock(x, y, xform, grid);
            }
        }
    }
}

fn weight(grid: &Grid<Cell>) -> i64 {
    let h = grid.y_bounds().end - grid.y_bounds().start;
    grid.iter_with_coord()
        .filter(|(c,_,_)| *c == Cell::Round)
        .map(|(_,_,y)| h - y)
        .sum()
}

fn part1(input: &[String]) -> i64 {
    let mut grid = mkgrid(input);
    tilt(&mut grid, CDir::N);
    //grid.print(|c| c.to_char());
    weight(&grid)
}

fn grid_to_str(grid: &Grid<Cell>) -> String {
    grid.iter().map(|&c| Into::<char>::into(c)).collect()
}
fn str_to_grid(grid: &mut Grid<Cell>, s: &str) {
    let mut chars = s.chars();
    grid.iter_mut().for_each(|c| *c = chars.next().unwrap().into());
}

fn part2(input: &[String]) -> i64 {
    let mut grid = mkgrid(input);
    let mut list: Vec<String> = Vec::new();
    let mut found: HashMap<String,usize> = HashMap::new();
    let key = grid_to_str(&grid);
    list.push(key.clone());
    found.insert(key, 0);
    for cycles in 1.. {
        tilt(&mut grid, CDir::N);
        tilt(&mut grid, CDir::W);
        tilt(&mut grid, CDir::S);
        tilt(&mut grid, CDir::E);
        let key = grid_to_str(&grid);
        if let Some(start) = found.get(&key) {
            let len = cycles - start;
            let idx = (1_000_000_000 - start) % len + start;
            str_to_grid(&mut grid, &list[idx]);
            return weight(&grid);
        }
        else {
            list.push(key.clone());
            found.insert(key, cycles);
        }
    }
    panic!();
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
    fn day14_test() {
        let input: Vec<String> = test_input(include_str!("day14.testinput"));
        assert_eq!(part1(&input), 136);
        /*
        let mut grid = mkgrid(&input);
        tilt(&mut grid, CDir::N);
        tilt(&mut grid, CDir::W);
        tilt(&mut grid, CDir::S);
        tilt(&mut grid, CDir::E);
        grid.print(|c| c.to_char());
        println!("");
        tilt(&mut grid, CDir::N);
        tilt(&mut grid, CDir::W);
        tilt(&mut grid, CDir::S);
        tilt(&mut grid, CDir::E);
        grid.print(|c| c.to_char());
        */
        assert_eq!(part2(&input), 64);
    }
}
