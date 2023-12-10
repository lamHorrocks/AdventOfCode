use std::fs::read_to_string;

mod camel_cards;
use crate::camel_cards::hand::*;

fn main() {
  let input: Vec<String> = read_to_string("input.txt")
    .expect("Could not read input file")
    .lines()
    .map(String::from)
    .collect();

  let mut hands: Vec<Hand> = Vec::new();

  for line in input {
    let mut split = line.split_whitespace();
    hands.push(Hand { cards: split.next().unwrap().to_string(), bid: split.next().unwrap().parse::<u32>().unwrap() });
  }

  hands.sort_by(|hand1, hand2| hand1.cmp(hand2));

  let mut result: u32 = 0;
  for (idx, hand) in hands.iter().enumerate() {
    result += (idx as u32 + 1) * hand.bid;
  }

  println!("{result}");
}
