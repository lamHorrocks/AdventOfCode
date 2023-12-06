use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
  id: u32,
  copies: u32,
  winning_numbers: Vec<u32>,
  numbers: Vec<u32>,
}

impl Card {
  fn points(&self) -> u32 {
    let mut points: u32 = 0;

    for i in &self.numbers {
      if self.winning_numbers.contains(&i) {
        if points == 0 {
          points = 1
        } else {
          points *= 2;
        }
      }
    }

    points
  }

  fn count_matches(&self) -> u32 {
    let mut count = 0;

    for i in &self.numbers {
      if self.winning_numbers.contains(&i) {
        count += 1;
      }
    }

    count
  }

  fn clone(&self) -> Card {
    Card {
      id: self.id.clone(),
      copies: self.copies.clone(),
      winning_numbers: self.winning_numbers.clone(),
      numbers: self.numbers.clone()
    }
  }
}

fn main() {
  let input: Vec<String> = read_to_string("input.txt")
    .expect("Could not read input")
    .lines()
    .map(String::from)
    .collect();

  let mut cards: Vec<Card> = vec![];
  for line in input {
    cards.push(parse_card_from_line(&line));
  }

  let mut total_points = 0;
  cards.iter().for_each(|card| total_points += card.points());

  //Part 1
  println!("Total Points {total_points}");

  let mut total_scratch_cards = 0;

  for i in 0..cards.len() {
    total_scratch_cards += cards[i].copies;
    let matches = cards[i].count_matches();

    for j in 1..=matches as usize {
      cards[i + j].copies += cards[i].copies;
    }
  }

  //Part 2
  println!("Total Scratch Cards: {total_scratch_cards}");
}

fn parse_card_from_line(line: &str) -> Card {
  let colon_idx = line.find(':').expect("Could not find colon in line");
  let separator_idx = line.find('|').expect("Could not find separator");

  let id: u32 = line.split(':')
    .next()
    .and_then(|part| part.split_whitespace()
    .last())
    .expect("Could not split Card ID part of string")
    .parse::<u32>()
    .expect("Could not parse card id");

  let winning_numbers_str: String = line[colon_idx + 1..separator_idx].to_string();
  let numbers_str: String = line[separator_idx + 1..].to_string();

  let winning_numbers: Vec<u32> = winning_numbers_str.split_whitespace()
    .map(|str| str.parse::<u32>().expect("Could not parse winning number to int"))
    .collect();

  let numbers: Vec<u32> = numbers_str.split_whitespace()
    .map(|str| str.parse::<u32>().expect("Could not parse number to int"))
    .collect();

  return Card {
    id,
    copies: 1,
    winning_numbers,
    numbers,
  }
}
