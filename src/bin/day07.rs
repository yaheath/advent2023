use std::cmp::Ordering;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Card {
    CW,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl Card {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Card::C2),
            '3' => Some(Card::C3),
            '4' => Some(Card::C4),
            '5' => Some(Card::C5),
            '6' => Some(Card::C6),
            '7' => Some(Card::C7),
            '8' => Some(Card::C8),
            '9' => Some(Card::C9),
            'T' => Some(Card::CT),
            'J' => Some(Card::CJ),
            'Q' => Some(Card::CQ),
            'K' => Some(Card::CK),
            'A' => Some(Card::CA),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HandType {
    HC = 0,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        let hand_type = get_hand_type(&cards);
        Self { cards, hand_type }
    }
    fn set_wild(&self) -> Self {
        let mut cards = self.cards.clone();
        for c in cards.iter_mut() {
            if *c == Card::CJ {
                *c = Card::CW;
            }
        }
        let hand_type = get_hand_type(&cards);
        Self { cards, hand_type }
    }
}
impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().flat_map(|c| Card::from_char(c)).collect::<Vec<_>>();
        if cards.len() == 5 {
            Ok(Hand::new(
                [cards[0], cards[1], cards[2], cards[3], cards[4]],
            ))
        }
        else {
            Err(())
        }
    }
}

fn get_hand_type(cards: &[Card; 5]) -> HandType {
    let mut counts = cards.iter()
        .sorted()
        .dedup_with_count()
        .sorted()
        .collect::<Vec<_>>();

    if let Some(idx) = counts.iter().position(|tup| *tup.1 == Card::CW) {
        let nwilds = counts[idx].0;
        if nwilds >= 4 { return HandType::Five }
        counts = counts.into_iter().filter(|tup| *tup.1 != Card::CW).collect();
        let end = counts.len() - 1;
        let tup = counts[end];
        counts[end] = (tup.0 + nwilds, tup.1);
    }
    if counts.len() == 1 {
        HandType::Five
    }
    else if counts.len() == 2 {
        if counts[1].0 == 4 {
            HandType::Four
        }
        else {
            HandType::FullHouse
        }
    }
    else if counts.len() == 3 {
        if counts[2].0 == 3 {
            HandType::Three
        }
        else {
            HandType::TwoPair
        }
    }
    else if counts.len() == 4 {
        HandType::OnePair
    }
    else {
        HandType::HC
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Input {
    hand: Hand,
    bid: usize,
}
impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut itr = s.split(' ');
        let hand = itr.next().unwrap().parse::<Hand>().unwrap();
        let bid = itr.next().unwrap().parse::<usize>().unwrap();
        Ok(Input {hand, bid})
    }
}

fn part1(input: &Vec<Input>) -> usize {
    let mut input: Vec<Input> = input.clone();
    input.sort_unstable_by(|a, b| a.hand.cmp(&b.hand));
    input.iter()
        .enumerate()
        .map(|(idx, inp)| inp.bid * (idx + 1))
        .sum()
}

fn part2(input: &Vec<Input>) -> usize {
    let mut input: Vec<Input> = input.iter().map(|i|
        Input { hand: i.hand.set_wild(), bid: i.bid }).collect();
    input.sort_unstable_by(|a, b| a.hand.cmp(&b.hand));
    input.iter()
        .enumerate()
        .map(|(idx, inp)| inp.bid * (idx + 1))
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
    use ya_advent_lib::read::test_input;

    #[test]
    fn day07_test() {
        let input: Vec<Input> = test_input(include_str!("day07.testinput"));
        assert_eq!(part1(&input), 6440);
        assert_eq!(part2(&input), 5905);
    }
}
