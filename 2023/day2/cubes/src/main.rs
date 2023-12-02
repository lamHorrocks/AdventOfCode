use std::{fs::read_to_string, u32::MAX};
use regex::Regex;

#[derive(Debug)]
struct Game {
  id: u32,
  red: u32,
  green: u32,
  blue: u32,
}

impl Game {
  fn violates_rule(&self, rule_game: &Game) -> bool {
    self.red > rule_game.red || self.green > rule_game.green || self.blue > rule_game.blue
  }
}

fn main() {
  let rule_game = Game {
    id: MAX,
    red: 12,
    green: 13,
    blue: 14
  };

  let lines: Vec<String> = read_to_string("input.txt")
    .expect("Could not read input file")
    .lines()
    .map(String::from)
    .collect();

  let mut id_sum: u32 = 0;
  let mut cube_power_sum = 0;

  for line in lines {
    if let Some(game) = parse_game_line(&line) {
      if !game.violates_rule(&rule_game) {
        id_sum += game.id
      }

      cube_power_sum += game.red * game.green * game.blue;
    } else {
      panic!("Could not process: {line}");
    }
  }

  //Part 1
  println!("ID Sum: {id_sum}");
  
  //Part 2
  println!("Cube Power Sum: {cube_power_sum}");
}

fn parse_game_line(game_str: &str) -> Option<Game> {
  let re_id = Regex::new(r"Game\s(\d+):").unwrap();
  let re_red = Regex::new(r"(\d+)\sred").unwrap();
  let re_blue = Regex::new(r"(\d+)\sblue").unwrap();
  let re_green = Regex::new(r"(\d+)\sgreen").unwrap();

  let Some(id_cap) = re_id.captures(game_str) else {
    println!("Could not capture id");
    return None;
  };

  let id = id_cap[1].parse::<u32>().unwrap();

  let reds: Vec<u32> = re_red.captures_iter(game_str)
    .filter_map(|cap| cap[1].parse::<u32>().ok())
    .collect();

  let greens: Vec<u32> = re_green.captures_iter(game_str)
    .filter_map(|cap| cap[1].parse::<u32>().ok())
    .collect();

  let blues: Vec<u32> = re_blue.captures_iter(game_str)
    .filter_map(|cap| cap[1].parse::<u32>().ok())
    .collect();

  let Some(max_red) = reds.iter().max() else {
    println!("No reds found");
    return None;
  };

  let Some(max_blue) = blues.iter().max() else {
    println!("No blues found");
    return None;
  };

  let Some(max_green) = greens.iter().max() else {
    println!("No greens found");
    return None;
  };

  Some(Game {
    id: id,
    red: *max_red,
    green: *max_green,
    blue: *max_blue,
  })
}
