use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::vec::Vec;
use num::integer::lcm;
use ya_advent_lib::read::read_input;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Other,
}

#[derive(Clone, Debug)]
struct Module {
    name: String,
    typ: ModuleType,
    targets: Vec<String>,
    inputs: HashMap<String, usize>,
    state: usize,
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(" -> ");
        let mut name = itr.next().unwrap();
        let typ = match name.chars().next().unwrap() {
            '%' => ModuleType::FlipFlop,
            '&' => ModuleType::Conjunction,
            'b' => ModuleType::Broadcaster,
            _ => panic!(),
        };
        if typ != ModuleType::Broadcaster {
            name = &name[1..];
        }
        let targets = itr.next().unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(Module{
            name: name.into(),
            typ,
            targets,
            inputs: HashMap::new(),
            state: 0,
        })
    }
}

impl Module {
    fn new(name: String, typ: ModuleType) -> Self {
        Self {
            name,
            typ,
            targets: Vec::new(),
            inputs: HashMap::new(),
            state: 0,
        }
    }
    fn set_state(&mut self, bitnum: usize, on: bool) {
        if on {
            self.state |= 1<<bitnum;
        }
        else {
            self.state &= !(1<<bitnum);
        }
    }
}

fn setup(input: &[Module]) -> HashMap<String,Module> {
    let mut out: HashMap<String,Module> = HashMap::new();
    for i in input {
        let m = i.clone();
        out.insert(i.name.clone(), m);
    }
    for i in input {
        for o in &i.targets {
            out.entry(o.clone())
                .and_modify(|t| {
                    let n = t.inputs.len();
                    t.inputs.insert(i.name.clone(), n);
                })
                .or_insert(Module::new(o.clone(), ModuleType::Other));
        }
    }
    out
}

#[allow(clippy::type_complexity)]
struct System<'a> {
    modules: HashMap<String,Module>,
    steps: usize,
    n_low: usize,
    n_high: usize,
    breakpoints: HashMap<String, Box<dyn FnMut(String, bool, usize) + 'a>>,
}

impl<'a> System<'a> {
    fn new(modules: HashMap<String,Module>) -> Self {
        Self {
            modules,
            steps: 0,
            n_low: 0,
            n_high: 0,
            breakpoints: HashMap::new(),
        }
    }
    fn step(&mut self) {
        self.steps += 1;
        let mut queue: VecDeque<(String,String,bool)> = VecDeque::new();
        queue.push_back(("broadcaster".into(), String::new(), false));

        while let Some((name, sender, pulse)) = queue.pop_front() {
            let m = self.modules.get_mut(&name).unwrap();
            if pulse { self.n_high += 1; } else { self.n_low += 1; }
            let send = match m.typ {
                ModuleType::Broadcaster => Some(pulse),
                ModuleType::FlipFlop => {
                    if !pulse {
                        m.set_state(0, m.state == 0);
                        if m.state == 0 { Some(false) } else { Some(true) }
                    }
                    else {
                        None
                    }
                },
                ModuleType::Conjunction => {
                    let idx = m.inputs[&sender];
                    m.set_state(idx, pulse);
                    let all_on = 2usize.pow(m.inputs.len() as u32) - 1;
                    if m.state == all_on {
                        Some(false)
                    } else {
                        Some(true)
                    }
                },
                ModuleType::Other => None,
            };
            if let Some(p) = send {
                if let Some(cb) = self.breakpoints.get_mut(&m.name) {
                    cb(m.name.clone(), p, self.steps);
                }
                for t in &m.targets {
                    //println!("{} (s={}) {}-> {}", m.name, m.state, p, t);
                    queue.push_back((t.clone(), m.name.clone(), p));
                }
            }
        }
    }
    fn run_for(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }
}

fn part1(input: &[Module]) -> usize {
    let modules = setup(input);
    let mut system = System::new(modules);
    system.run_for(1000);
    system.n_low * system.n_high
}

fn part2(input: &[Module]) -> usize {
    let modules = setup(input);
    /*
    println!("digraph modules {{");
    modules.values().for_each(|m| {
        m.targets.iter().for_each(|t| {
            println!("{} -> {}", m.name, t);
        });
    });
    println!("}}");
    */

    /*
     * From looking at the graph, the rx node is fed from a Conjuction node
     * which in turn is fed from four other Conjuction nodes, and each of those
     * is fed from a network of nodes. Those four networks do not have
     * any interconnections, other than a connection to the broadcaster.
     *
     * So each network generates a high pulse out of its output Conjunction node
     * at a repeating interval. So we detect the high pulses and store the step
     * number for each, and once they've all pulsed we can LCM them to get the
     * time in the (far) future when they all go high at the same time.
     */

    let rx_feeder = modules.values().find(|m| m.targets.len() == 1 && m.targets[0] == "rx").unwrap();
    let counters:HashMap<String,usize> = HashMap::from_iter(
        rx_feeder.inputs.keys().map(|n| (n.clone(), 0))
    );
    let counters: RefCell<HashMap<String,usize>> = RefCell::new(counters);
    let cb = &|name, hi, steps| {
        if hi {
            let mut c = counters.borrow_mut();
            c.entry(name).and_modify(|v| if *v == 0 { *v = steps; });
        }
    };
    let mut system = System::new(modules);
    for n in counters.borrow().keys() {
        system.breakpoints.insert(n.clone(), Box::new(cb));
    }
    for _ in 1.. {
        system.step();
        if counters.borrow().values().all(|v| *v > 0) {
            return counters.borrow().values().copied().reduce(lcm).unwrap();
        }
    }
    unreachable!();
}

fn main() {
    let input: Vec<Module> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day20_test() {
        let input: Vec<Module> = test_input(
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
");
        assert_eq!(part1(&input), 32000000);

        let input: Vec<Module> = test_input(
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
");
        assert_eq!(part1(&input), 11687500);
    }
}
