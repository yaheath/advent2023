use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |acc, n| ((acc + n) * 17) & 255)
}

fn part1(input: &[String]) -> usize {
    input.iter()
        .map(|s| s.split(',').map(hash).sum::<usize>())
        .sum()
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal: usize,
}

#[derive(Clone)]
struct LensBox {
    lenses: Vec<Lens>
}

impl LensBox {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }
    fn insert(&mut self, lens: Lens) {
        if let Some(idx) = self.lenses.iter().position(|l| l.label == lens.label) {
            self.lenses[idx] = lens;
        }
        else {
            self.lenses.push(lens);
        }
    }
    fn remove(&mut self, label: &str) {
        self.lenses.retain(|l| l.label != label);
    }
}

enum Action {
    Add(usize),
    Remove,
}

struct Input {
    label: String,
    action: Action,
}
impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split('=').collect::<Vec<_>>();
        let mut label = String::from(v[0]);
        let action = if v.len() == 1 {
            label.pop();
            Action::Remove
        }
        else {
            Action::Add(v[1].parse::<usize>().unwrap())
        };
        Ok(Self { label, action })
    }
}

fn part2(input: &[String]) -> usize {
    let mut boxes = vec![LensBox::new(); 256];
    input.iter()
        .flat_map(|s| s.split(',').map(|ss| ss.parse::<Input>().unwrap()))
        .for_each(|i| {
            let h = hash(&i.label);
            match i.action {
                Action::Add(n) => boxes[h].insert(Lens { label: i.label, focal: n }),
                Action::Remove => boxes[h].remove(&i.label),
            }
        });
    /*
    boxes.iter().enumerate().filter(|(_,b)| b.lenses.len() > 0).for_each(|(idx, b)| {
        print!("box {idx}: ");
        b.lenses.iter().for_each(|l| { print!("{} {}, ", l.label, l.focal); });
        println!("");
    });
    */
    boxes.iter().enumerate()
        .map(|(box_idx, b)| b.lenses.iter().enumerate()
            .map(|(l_idx, lens)| (box_idx + 1) * (l_idx + 1) * lens.focal)
            .sum::<usize>()
        )
        .sum()
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
    fn day15_test() {
        let input: Vec<String> = test_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(part1(&input), 1320);
        assert_eq!(part2(&input), 145);
    }
}
