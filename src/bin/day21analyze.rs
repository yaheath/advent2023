use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::algorithm::a_star;

mod day21;

fn pathlen(grid: &Grid<day21::Cell>, start: Coord2D, end: Coord2D) -> usize {
    a_star(
        start,
        |loc| loc == end,
        |loc| loc.neighbors4().iter()
            .filter(|n| grid.contains_coord(**n) && grid.get_c(**n) != day21::Cell::Rock)
            .map(|n| (*n, 1))
            .collect::<Vec<_>>(),
        |loc| loc.mdist_to(&end) as usize,
    ).unwrap()
}

fn main() {
    let input: Vec<String> = read_input();
    let grid = day21::mkgrid(&input);
    let (x, y) = grid.find(|c,_,_| c == day21::Cell::Start).unwrap();
    assert_eq!(x, (grid.x_bounds().end - grid.x_bounds().start) / 2);
    assert_eq!(y, (grid.y_bounds().end - grid.y_bounds().start) / 2);
    assert_eq!(x, y);
    let center = Coord2D::new(x, y);
    let size = grid.x_bounds().end - grid.x_bounds().start;

    println!("steps from center to edges (where different from manhattan dist):");
    for x in grid.x_bounds() {
        let dest = Coord2D::new(x, 0);
        let p = pathlen(&grid, center, dest) as i64;
        let md = dest.mdist_to(&center);
        if p != md {
            println!("{},{}: {} {}", dest.x, dest.y, p, p - md);
        }
    }
    for y in grid.y_bounds() {
        let dest = Coord2D::new(y, 0);
        let p = pathlen(&grid, center, dest) as i64;
        let md = dest.mdist_to(&center);
        if p != md {
            println!("{},{}: {} {}", dest.x, dest.y, p, p - md);
        }
    }
    for x in grid.x_bounds() {
        let dest = Coord2D::new(x, size-1);
        let p = pathlen(&grid, center, dest) as i64;
        let md = dest.mdist_to(&center);
        if p != md {
            println!("{},{}: {} {}", dest.x, dest.y, p, p - md);
        }
    }
    for y in grid.y_bounds() {
        let dest = Coord2D::new(y, size-1);
        let p = pathlen(&grid, center, dest) as i64;
        let md = dest.mdist_to(&center);
        if p != md {
            println!("{},{}: {} {}", dest.x, dest.y, p, p - md);
        }
    }
    let start = Coord2D::new(x, 0);
    let n = day21::fill(&grid, start, 0, 1000000).len();
    println!("\nfill from center: {n}");
    println!("\nfill from edges");
    for x in grid.x_bounds() {
        let start = Coord2D::new(x, 0);
        let n_even = day21::fill(&grid, start, 0, 1000000).len();
        let n_odd = day21::fill(&grid, start, 1, 1000000).len();
        println!("{},{}: e: {n_even} o: {n_odd}", start.x, start.y);
    }
    for y in grid.y_bounds() {
        let start = Coord2D::new(0, y);
        let n_even = day21::fill(&grid, start, 0, 1000000).len();
        let n_odd = day21::fill(&grid, start, 1, 1000000).len();
        println!("{},{}: e: {n_even} o: {n_odd}", start.x, start.y);
    }
    for x in grid.x_bounds() {
        let start = Coord2D::new(x, size-1);
        let n_even = day21::fill(&grid, start, 0, 1000000).len();
        let n_odd = day21::fill(&grid, start, 1, 1000000).len();
        println!("{},{}: e: {n_even} o: {n_odd}", start.x, start.y);
    }
    for y in grid.y_bounds() {
        let start = Coord2D::new(size-1, y);
        let n_even = day21::fill(&grid, start, 0, 1000000).len();
        let n_odd = day21::fill(&grid, start, 1, 1000000).len();
        println!("{},{}: e: {n_even} o: {n_odd}", start.x, start.y);
    }

}
