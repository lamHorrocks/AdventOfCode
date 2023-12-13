use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
  let input: Vec<String> = read_to_string("input.txt")
    .expect("Could not read input file")
    .lines()
    .map(String::from)
    .collect();

  let mut directions: String = String::from("");
  let mut node_map: HashMap<String, (String, String)> = HashMap::new();

  for line in input {
    if !line.is_empty() {
      if line.contains('=') {
        let mut split = line.split('=');
        let node = String::from(split.next().unwrap().trim());
        let paths: String = split.next()
          .unwrap()
          .trim()
          .chars()
          .filter(|c| *c != '(' && *c != ')' && *c != ' ')
          .collect();

        let mut split = paths.split(',');
        let left = String::from(split.next().unwrap());
        let right = String::from(split.next().unwrap());

        node_map.insert(node, (left, right));

      } else {
        directions = line;
      }
    }
  }

  let mut node = node_map.get("AAA").unwrap();
  let mut steps: u32 = 0;

  for c in directions.chars().cycle() {
    if c == 'L' {
      steps += 1;

      if node.0 == "ZZZ" {
        break;
      }
      
      node = node_map.get(&node.0).unwrap();
    } else if c == 'R' {
      steps += 1;

      if node.1 == "ZZZ" {
        break;
      }

      node = node_map.get(&node.1).unwrap();
    } else {
      panic!("Found invalid direction: {c}");
    }
  }

  println!("Steps: {steps}");
}
