use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::Coord3D;

#[derive(Clone)]
struct Brick {
    blocks: HashSet<Coord3D>,
    pos: Coord3D,
    //size: Coord3D,
    supports: HashSet<BrickIndex>,
    supported_by: HashSet<BrickIndex>,
}

impl FromStr for Brick {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once('~') {
            let mut blocks = HashSet::new();
            let a = a.parse::<Coord3D>().unwrap();
            let b = b.parse::<Coord3D>().unwrap();
            assert!(a.x <= b.x && a.y <= b.y && a.z <= b.z);
            for x in a.x ..= b.x {
                for y in a.y ..= b.y {
                    for z in a.z ..= b.z {
                        blocks.insert(Coord3D::new(x,y,z) - a);
                    }
                }
            }
            Ok(Brick{
                blocks,
                pos: a,
                //size: b - a + Coord3D::new(1,1,1),
                supports: HashSet::new(),
                supported_by: HashSet::new(),
            })
        }
        else {
            Err(())
        }
    }
}

impl Brick {
    fn world_coords(&self) -> impl Iterator<Item=Coord3D> + '_ {
        self.blocks.iter().map(|c| *c + self.pos)
    }
}

type BrickIndex = usize;
struct Volume {
    bricks: Vec<Brick>,
    ordered: Vec<BrickIndex>,
    blocks: HashMap<Coord3D, BrickIndex>,
}
impl Volume {
    fn new(input: &Vec<Brick>) -> Self {
        let bricks = input.clone();
        let mut ordered = Vec::from_iter(0..bricks.len());
        ordered.sort_by_key(|idx| bricks[*idx].pos.z);
        let blocks = HashMap::from_iter(
            input.iter()
            .enumerate()
            .flat_map(|(idx, b)| b.world_coords().map(move |c| (c, idx)))
        );
        //println!("total blocks: {}", blocks.len());
        Self { bricks, blocks, ordered }
    }

    fn drop(&mut self) {
        for idx in self.ordered.clone() {
            self.drop_brick(idx);
        }
        //println!("total blocks after drop: {}", self.blocks.len());
        self.ordered.sort_by_key(|idx| self.bricks[*idx].pos.z);
    }

    fn drop_brick(&mut self, idx: BrickIndex) {
        let brick = self.bricks.get_mut(idx).unwrap();
        let foot = brick.blocks.iter().filter(|c| c.z == 0).collect::<Vec<_>>();

        brick.world_coords().for_each(|c| {self.blocks.remove(&c);});
        while brick.pos.z > 0 {
            let nc = brick.pos - Coord3D::z();
            brick.supported_by = HashSet::from_iter(
                foot.iter().map(|&&c| c + nc).filter_map(|c| self.blocks.get(&c)).copied()
            );
            if brick.supported_by.len() > 0 {
                break;
            }
            brick.pos = nc;
        }
        brick.world_coords().for_each(|c| {self.blocks.insert(c, idx);});
        for sup in brick.supported_by.clone() {
            let sb = self.bricks.get_mut(sup).unwrap();
            sb.supports.insert(idx);
        }
    }

    fn would_fall(&self, idx: BrickIndex) -> usize {
        let mut queue: VecDeque<BrickIndex> = VecDeque::new();
        let mut falling: HashSet<BrickIndex> = HashSet::new();
        queue.push_back(idx);
        falling.insert(idx);
        while let Some(idx) = queue.pop_front() {
            let brick = &self.bricks[idx];
            brick.supports.iter().for_each(|sup| {
                let sb = &self.bricks[*sup];
                if !falling.contains(sup) && sb.supported_by.iter().all(|b| falling.contains(b)) {
                    queue.push_back(*sup);
                    falling.insert(*sup);
                }
            });
        }
        falling.len() - 1       // don't count the idx brick itself
    }
}

fn setup(input: &Vec<Brick>) -> Volume {
    let mut volume = Volume::new(input);
    volume.drop();
    volume
}

fn part1(volume: &Volume) -> usize {
    volume.bricks.iter()
        .filter(|brick|
            !brick.supports.iter().any(|sb| volume.bricks[*sb].supported_by.len() == 1)
        )
        .count()
}

fn part2(volume: &Volume) -> usize {
    (0..volume.bricks.len())
        .map(|idx| volume.would_fall(idx))
        .sum()
}

fn main() {
    let input: Vec<Brick> = read_input();
    let volume = setup(&input);
    println!("Part 1: {}", part1(&volume));
    println!("Part 2: {}", part2(&volume));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day22_test() {
        let input: Vec<Brick> = test_input(include_str!("day22.testinput"));
        let volume = setup(&input);
        assert_eq!(part1(&volume), 5);
        assert_eq!(part2(&volume), 7);
    }
}
