use std::ops::RangeInclusive;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::geom::{Point2D, Point3D, Ray2D};
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone)]
struct Stone {
    pos: Point3D,
    vel: Point3D,
}

impl FromStr for Stone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" @ ").unwrap();
        let pos = p.parse::<Point3D>().unwrap();
        let vel = v.parse::<Point3D>().unwrap();
        Ok(Stone{pos, vel})
    }
}

impl Stone {
    fn intersect_2d(&self, other: &Stone, range: &RangeInclusive<f64>) -> bool {
        let a = Ray2D::new(Point2D::new(self.pos.x, self.pos.y), Point2D::new(self.vel.x, self.vel.y));
        let b = Ray2D::new(Point2D::new(other.pos.x, other.pos.y), Point2D::new(other.vel.x, other.vel.y));
        if let Some(c) = a.intersect_with(&b) {
            range.contains(&c.x) && range.contains(&c.y)
        }
        else {
            false
        }
    }
}

fn num_2d_intersections(input: &[Stone], range: RangeInclusive<f64>) -> usize {
    input.iter()
        .tuple_combinations()
        .filter(|(a, b)| a.intersect_2d(b, &range))
        .count()
}

fn part1(input: &[Stone]) -> usize {
    num_2d_intersections(input, 200000000000000.0..=400000000000000.0)
}

fn independent(a: Point3D, b: Point3D) -> bool {
    let c = a.cross(b);
    c.x != 0.0 || c.y != 0.0 || c.z != 0.0
}

fn find_plane(s1: &Stone, s2: &Stone) -> (Point3D, f64) {
    let pdiff = s1.pos - s2.pos;
    let vdiff = s1.vel - s2.vel;
    let vv = s1.vel.cross(s2.vel);
    (pdiff.cross(vdiff), pdiff.dot(vv))
}

fn lin(r: f64, a: Point3D, s: f64, b: Point3D, t: f64, c: Point3D) -> Point3D {
    Point3D::new(
         r * a.x + s * b.x + t * c.x,
         r * a.y + s * b.y + t * c.y,
         r * a.z + s * b.z + t * c.z,
    )
}

fn part2(input: &[Stone]) -> i64 {
    /* this comes from:
https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kersplf/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    */
    let (stone_a, stone_b, stone_c) = input.iter()
        .tuple_combinations()
        .find(|(a, b, c)| independent(a.vel, b.vel) && independent(a.vel, c.vel) && independent(b.vel, c.vel))
        .unwrap();
    let (a, aa) = find_plane(stone_a, stone_b);
    let (b, bb) = find_plane(stone_a, stone_c);
    let (c, cc) = find_plane(stone_b, stone_c);

    let w = lin(aa, b.cross(c), bb, c.cross(a), cc, a.cross(b));
    let t = a.dot(b.cross(c));

    let w = Point3D::new((w.x / t).round(), (w.y / t).round(), (w.z / t).round());
    let w1 = stone_a.vel - w;
    let w2 = stone_b.vel - w;
    let ww = w1.cross(w2);

    let ee = ww.dot(stone_b.pos.cross(w2));
    let ff = ww.dot(stone_a.pos.cross(w1));
    let gg = stone_a.pos.dot(ww);
    let ss = ww.dot(ww);

    let rock = lin(ee, w1, -ff, w2, gg, ww);
    ((rock.x + rock.y + rock.z) / ss) as i64
}

fn main() {
    let input: Vec<Stone> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day24_test() {
        let input: Vec<Stone> = test_input(include_str!("day24.testinput"));
        assert_eq!(num_2d_intersections(&input, 7.0..=27.0), 2);
        assert_eq!(part2(&input), 47);
    }
}
