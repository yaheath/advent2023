use std::collections::{HashMap, VecDeque};
use std::ops::Range;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_grouped_input;
use advent_lib::iter::FirstLast;

#[derive(Debug)]
struct SeedMapEntry {
    from: Range<u64>,
    to: u64
}

impl FromStr for SeedMapEntry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split(' ').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        Ok(Self{
            from: nums[1] .. nums[1] + nums[2],
            to: nums[0],
        })
    }
}

impl SeedMapEntry {
    fn map(&self, id: u64) -> Option<u64> {
        if self.from.contains(&id) {
            Some(self.to + (id - self.from.start))
        }
        else {
            None
        }
    }
    fn len(&self) -> u64 {
        self.from.end - self.from.start
    }
}

struct Input {
    seeds: Vec<u64>,
    seeds_ranges: Vec<Range<u64>>,
    path: HashMap<String,String>,
    maps: HashMap<String,Vec<SeedMapEntry>>,
}

impl Input {
    fn from_input(input: Vec<Vec<String>>) -> Self {
        let seeds = input[0][0].split(' ').skip(1).map(|s| s.parse::<u64>().unwrap()).collect();
        let seeds_ranges = input[0][0].split(' ').skip(1)
            .map(|s| s.parse::<u64>().unwrap())
            .tuples()
            .map(|(f, c)| f .. f+c)
            .collect();
        let mut path = HashMap::new();
        let mut maps = HashMap::new();
        for sect in input.iter().skip(1) {
            let (frm, to) = sect[0].split(' ').next().unwrap().split("-to-").first_last().unwrap();
            path.insert(frm.into(), to.into());
            let mut v:Vec<SeedMapEntry> = sect.iter()
                .skip(1)
                .map(|s| s.parse::<SeedMapEntry>().unwrap())
                .collect();
            v.sort_by_key(|r| r.from.start);
            maps.insert(frm.into(), v);
        }
        Self { seeds, seeds_ranges, path, maps }
    }

    fn map_item(&self, frm: &str, to: &str, frm_id: u64) -> u64 {
        let mut id = frm_id;
        let mut current = frm;
        while current != to {
            let next = &self.path[current];
            let map = &self.maps[current];
            if let Some(next_id) = map.iter().find_map(|m| m.map(id)) {
                id = next_id;
            }
            current = next;
        }
        id
    }

    fn map_range(&self, frm: &str, to: &str, range: Range<u64>) -> Vec<Range<u64>> {
        let mut cur_range = VecDeque::new();
        cur_range.push_back(range);
        let mut current = frm;
        while current != to {
            let next = &self.path[current];
            let map = &self.maps[current];
            let mut next_range = Vec::new();
            while let Some(r) = cur_range.pop_front() {
                if let Some(mtch) = map.iter().find(|m| m.from.contains(&r.start)) {
                    let to_start = mtch.to + (r.start - mtch.from.start);
                    let len = (r.end - r.start).min(mtch.len() - (to_start - mtch.to));
                    next_range.push(to_start .. to_start + len);
                    if len < r.end - r.start {
                        cur_range.push_front(r.start + len .. r.end);
                    }
                }
                else {
                    if let Some(nxt) = map.iter().find(|m| m.from.start > r.start) {
                        next_range.push(r.start .. r.end.min(nxt.from.start));
                        if r.end < nxt.from.start {
                            cur_range.push_front(r.end .. nxt.from.start);
                        }
                    }
                    else {
                        next_range.push(r);
                    }
                }
            }
            current = next;
            next_range.sort_by_key(|r| r.start);
            cur_range = next_range.into();
        }
        cur_range.into()
    }
}

fn part1(input: &Input) -> u64 {
    input.seeds.iter()
        .map(|s| input.map_item("seed", "location", *s))
        .min()
        .unwrap()
}

fn part2(input: &Input) -> u64 {
    input.seeds_ranges.iter()
        .cloned()
        .map(|sr| input.map_range("seed", "location", sr))
        .map(|v| v[0].start)
        .min()
        .unwrap()
}

fn main() {
    let input = Input::from_input(read_grouped_input());
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::grouped_test_input;

    #[test]
    fn day05_test() {
        let input = Input::from_input(grouped_test_input(include_str!("day05.testinput")));
        assert_eq!(part1(&input), 35);
        assert_eq!(part2(&input), 46);
    }
}
