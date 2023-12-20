use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_sectioned_input;

#[derive(Clone)]
enum Dest {
    Accept,
    Reject,
    Rule(String),
}

struct Rule {
    rating: char,
    op: char,
    val: usize,
    dest: Dest,
}

enum RuleOrDest {
    R(Rule),
    D(Dest),
}

struct Workflow {
    name: String,
    rules: Vec<RuleOrDest>,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Dest {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "A" {
            Ok(Dest::Accept)
        }
        else if s == "R" {
            Ok(Dest::Reject)
        }
        else {
            Ok(Dest::Rule(s.into()))
        }
    }
}

impl FromStr for RuleOrDest {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr1 = s.split(':');
        let first = itr1.next().unwrap();
        if let Some(dest) = itr1.next() {
            let mut itr2 = first.chars();
            let rating = itr2.next().unwrap();
            let op = itr2.next().unwrap();
            let val = itr2.collect::<String>().parse::<usize>().unwrap();
            let dest = dest.parse::<Dest>().unwrap();
            Ok(RuleOrDest::R(Rule{rating, op, val, dest}))
        }
        else {
            Ok(RuleOrDest::D(first.parse::<Dest>().unwrap()))
        }
    }
}

impl FromStr for Workflow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split('{');
        let name = itr.next().unwrap().into();
        let rules = itr.next()
            .map(|x| x.strip_suffix('}').unwrap())
            .map(|x| x.split(',').map(|y| y.parse::<RuleOrDest>().unwrap()).collect::<Vec<_>>())
            .unwrap();
        Ok(Workflow{name, rules})
    }
}

impl FromStr for Part {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s[1..].strip_suffix('}').unwrap().split(',');
        let x = itr.next().unwrap()[2..].parse::<usize>().unwrap();
        let m = itr.next().unwrap()[2..].parse::<usize>().unwrap();
        let a = itr.next().unwrap()[2..].parse::<usize>().unwrap();
        let s = itr.next().unwrap()[2..].parse::<usize>().unwrap();
        Ok(Part{x, m, a, s})
    }
}
impl Part {
    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        let v = match self.rating {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!(),
        };
        match self.op {
            '<' => v < self.val,
            '>' => v > self.val,
            _ => panic!(),
        }
    }
}

impl Part {
    fn apply_rules(&self, workflows: &HashMap<String, &Workflow>) -> Dest {
        let mut current = String::from("in");
        loop {
            let wf = workflows[&current];
            let r_or_d = wf.rules.iter().find(|rd| match rd {
                RuleOrDest::R(r) => r.matches(&self),
                RuleOrDest::D(_) => true,
            }).unwrap();
            let dest = match r_or_d {
                RuleOrDest::R(r) => r.dest.clone(),
                RuleOrDest::D(d) => d.clone(),
            };
            match dest {
                Dest::Rule(n) => {current = n;},
                _ => {return dest;},
            }
        }
    }
}

fn part1(input: &(Vec<Workflow>, Vec<Part>)) -> usize {
    let wfmap: HashMap<String, &Workflow> = HashMap::from_iter(
        input.0.iter().map(|i| (i.name.clone(), i))
    );
    input.1.iter()
        .map(|part| (part.apply_rules(&wfmap), part))
        .filter_map(|(result, part)| match result {
            Dest::Accept => Some(part.value()),
            _ => None,
        })
        .sum()
}

#[derive(Clone)]
struct Partition {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl Partition {
    fn new() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
    fn get(&self, r: char) -> Range<usize> {
        match r {
            'x' => self.x.clone(),
            'm' => self.m.clone(),
            'a' => self.a.clone(),
            's' => self.s.clone(),
            _ => panic!(),
        }
    }
    fn set(&self, r: char, val: Range<usize>) -> Self {
        let mut new = self.clone();
        match r {
            'x' => new.x = val,
            'm' => new.m = val,
            'a' => new.a = val,
            's' => new.s = val,
            _ => panic!(),
        }
        new
    }
    fn combinations(&self) -> usize {
        (self.x.end - self.x.start) *
        (self.m.end - self.m.start) *
        (self.a.end - self.a.start) *
        (self.s.end - self.s.start)
    }
}

fn traverse(
    wfmap: &HashMap<String, &Workflow>,
    cur_wf: String,
    cur_rule: usize,
    cur_partition: Partition,
) -> Vec<Partition> {
    let mut wf = wfmap[&cur_wf];
    let mut cur_rule = cur_rule;
    loop {
        match &wf.rules[cur_rule] {
            RuleOrDest::R(r) => {
                let range = cur_partition.get(r.rating);
                if r.op == '<' && range.end <= r.val
                  || r.op == '>' && range.start > r.val {
                    match &r.dest {
                        Dest::Accept => {
                            return vec![cur_partition];
                        },
                        Dest::Reject => {
                            return vec![];
                        },
                        Dest::Rule(r) => {
                            wf = wfmap[r];
                            cur_rule = 0;
                        },
                    }
                }
                else if r.op == '<' && range.start < r.val
                  || r.op == '>' && range.end > r.val + 1 {
                    let val = if r.op == '<' { r.val } else { r.val + 1 };
                    let r1 = range.start .. val;
                    let r2 = val .. range.end;
                    let p1 = cur_partition.set(r.rating, r1);
                    let p2 = cur_partition.set(r.rating, r2);
                    let mut a1 = traverse(wfmap, wf.name.clone(), cur_rule, p1);
                    let mut a2 = traverse(wfmap, wf.name.clone(), cur_rule, p2);
                    a1.append(&mut a2);
                    return a1;
                }
                else {
                    cur_rule += 1;
                }
            },
            RuleOrDest::D(d) => {
                match d {
                    Dest::Accept => {
                        return vec![cur_partition];
                    },
                    Dest::Reject => {
                        return vec![];
                    },
                    Dest::Rule(r) => {
                        wf = wfmap[r];
                        cur_rule = 0;
                    },
                }
            }
        }
    }
}

fn part2(input: &(Vec<Workflow>, Vec<Part>)) -> usize {
    let wfmap: HashMap<String, &Workflow> = HashMap::from_iter(
        input.0.iter().map(|i| (i.name.clone(), i))
    );
    traverse(&wfmap, "in".into(), 0, Partition::new())
        .iter()
        .map(|v| v.combinations())
        .sum()
}

fn main() {
    let input: (Vec<Workflow>, Vec<Part>) = read_sectioned_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::sectioned_test_input;

    #[test]
    fn day19_test() {
        let input: (Vec<Workflow>, Vec<Part>) = sectioned_test_input(include_str!("day19.testinput"));
        assert_eq!(part1(&input), 19114);
        assert_eq!(part2(&input), 167409079868000);
    }
}
