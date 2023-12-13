use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Good,
    Bad,
    Unknown,
}

struct Input {
    springs: Vec<State>,
    counts: Vec<usize>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(' ');
        let springs = itr.next().unwrap().chars().map(|c| match c {
            '?' => State::Unknown,
            '#' => State::Good,
            '.' => State::Bad,
            _ => panic!(),
        }).collect();
        let counts = itr.next().unwrap()
            .split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        Ok(Input{ springs, counts })
    }
}
impl Input {
    fn expanded(&self) -> Self {
        let mut springs = Vec::with_capacity(self.springs.len()*5 + 4);
        let mut counts = Vec::with_capacity(self.counts.len()*5);
        for n in 0..5 {
            springs.extend(self.springs.iter());
            if n < 4 {
                springs.push(State::Unknown);
            }
            counts.extend(self.counts.iter());
        }
        Self { springs, counts }
    }
}

struct Solver<'a> {
    springs: &'a Vec<State>,
    counts: &'a Vec<usize>,
    cache: RefCell<HashMap<(usize, usize, usize), usize>>,
}

impl<'a> Solver<'a> {
    fn new(input: &'a Input) -> Self {
        Self {
            springs: &input.springs,
            counts: &input.counts,
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn solve(&self) -> usize {
        self.search(0, 0, 0)
    }

    fn search(&self, spr_idx:usize, cnt_idx:usize, current_cnt:usize) -> usize {
        let key = (spr_idx, cnt_idx, current_cnt);
        if let Some(val) = self.cache.borrow().get(&key) {
            return *val;
        }
        if spr_idx == self.springs.len() {
            if cnt_idx == self.counts.len() && current_cnt == 0 {
                return 1;
            }
            if cnt_idx == self.counts.len() - 1 && self.counts[cnt_idx] == current_cnt {
                return 1;
            }
            return 0;
        }
        let mut total = 0;
        for st in [State::Bad, State::Good] {
            if self.springs[spr_idx] == st || self.springs[spr_idx] == State::Unknown {
                if st == State::Bad && current_cnt == 0 {
                    total += self.search(spr_idx + 1, cnt_idx, 0);
                }
                else if st == State::Bad &&
                        current_cnt > 0 &&
                        cnt_idx < self.counts.len() &&
                        self.counts[cnt_idx] == current_cnt {
                    total += self.search(spr_idx + 1, cnt_idx + 1, 0);
                }
                else if st == State::Good {
                    total += self.search(spr_idx + 1, cnt_idx, current_cnt + 1);
                }
            }
        }

        self.cache.borrow_mut().insert(key, total);
        total
    }
}

fn part1(input: &Vec<Input>) -> usize {
    input.iter()
        .map(|i| Solver::new(&i).solve())
        .sum()
}

fn part2(input: &Vec<Input>) -> usize {
    input.iter()
        .map(|i| i.expanded())
        .map(|i| Solver::new(&i).solve())
        .sum()
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day12_test() {
        let input: Vec<Input> = test_input(include_str!("day12.testinput"));
        assert_eq!(part1(&input), 21);
        assert_eq!(part2(&input), 525152);
    }
}
