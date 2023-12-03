use std::collections::HashMap;
use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::grid::Grid;

fn is_symbol(grid: &Grid<char>, x: i64, y: i64) -> bool {
    match grid.get(x, y) {
        '.' => false,
        '0'..='9' => false,
        _ => true,
    }
}

fn is_gear(grid: &Grid<char>, x: i64, y: i64) -> bool {
    grid.get(x, y) == '*'
}

fn get_number(grid: &Grid<char>, x: i64, y: i64, gear_map: &mut HashMap<(i64,i64),Vec<u64>>) -> (u64, bool, i64) {
    let mut s: String = String::new();
    let mut next_x = x;
    for x1 in x.. {
        let c = grid.get(x1, y);
        match c {
            '0'..='9' => { s.push(c); }
            _ => { next_x = x1; break; }
        }
    }
    let v = s.parse::<u64>().unwrap();
    let mut is_pn = false;
    for x1 in (x-1)..=next_x {
        if is_symbol(grid, x1, y - 1) {
            is_pn = true;
        }
        if is_symbol(grid, x1, y + 1) {
            is_pn = true;
        }
        if (x1 == x-1 || x1 == next_x) && is_symbol(grid, x1, y) {
            is_pn = true;
        }
        if is_gear(grid, x1, y - 1) {
            gear_map.entry((x1, y-1)).and_modify(|n| n.push(v)).or_insert(vec![v]);
        }
        if is_symbol(grid, x1, y + 1) {
            gear_map.entry((x1, y+1)).and_modify(|n| n.push(v)).or_insert(vec![v]);
        }
        if (x1 == x-1 || x1 == next_x) && is_symbol(grid, x1, y) {
            gear_map.entry((x1, y)).and_modify(|n| n.push(v)).or_insert(vec![v]);
        }
    }
    //println!("{x},{y}: {v} {is_pn} {next_x}");
    (v, is_pn, next_x)
}

fn bothparts(input: &Vec<String>) -> (u64, u64) {
    let grid: Grid<char> = Grid::from_input(input, '.', 1, |c| c);
    let mut sum = 0;
    let x_width = grid.x_bounds().end - 1;
    let y_width = grid.y_bounds().end - 1;
    let mut gear_map: HashMap<(i64,i64),Vec<u64>> = HashMap::new();
    for y in 0..y_width {
        let mut x = 0;
        while x < x_width {
            match grid.get(x, y) {
                '0'..='9' => {
                    let (n, is_pn, next_x) = get_number(&grid, x, y, &mut gear_map);
                    if is_pn {
                        sum += n;
                    }
                    x = next_x;
                },
                _ => { x += 1; },
            }
        }
    }
    let gears = gear_map.iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<u64>())
        .sum();
    (sum, gears)
}


fn main() {
    let input: Vec<String> = read_input();
    let (p1, p2) = bothparts(&input);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day03_test() {
        let input: Vec<String> = test_input(include_str!("day03.testinput"));
        assert_eq!(bothparts(&input), (4361, 467835));
    }
}
