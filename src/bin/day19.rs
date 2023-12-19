use std::collections::HashMap;
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
        let mut first = itr1.next().unwrap();
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

fn part2(input: &(Vec<Workflow>, Vec<Part>)) -> usize {
    0
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
        assert_eq!(part2(&input), 0);
    }
}
