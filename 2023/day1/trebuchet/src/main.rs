use std::fs::read_to_string;

fn main() {
  let lines: Vec<String> = read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(String::from)
    .collect();

  let mut sum: u32 = 0;

  for line in lines {
    let digits = parse_str_words(&line);
    sum += digits.first().unwrap() * 10 + digits.last().unwrap();
  }

  println!("{sum}");
}

//Part 1
// fn parse_str_digits(str: &str) -> Vec<u32> {
//   str
//     .chars()
//     .filter_map(|c| c.to_digit(10))
//     .collect()
// }

//Part 2
fn parse_str_words(str: &str) -> Vec<u32> {
  let mut result: Vec<u32> = vec![];
  let mut curr_word = String::from("");

  for c in str.chars() {
    if c.is_digit(10) {
      result.push(c.to_digit(10).unwrap());
    } else {
      curr_word.push(c);
      if let Some(num) = get_num_from_str(&curr_word) {
        result.push(num);
        curr_word.clear();
        curr_word.push(c);
      }
    }
  }

  result
}

fn get_num_from_str(str: &str) -> Option<u32> {
  let result: Option<u32> = match str.to_lowercase().as_str() {
    x if x.contains("one") => Some(1),
    x if x.contains("two") => Some(2),
    x if x.contains("three") => Some(3),
    x if x.contains("four") => Some(4),
    x if x.contains("five") => Some(5),
    x if x.contains("six") => Some(6),
    x if x.contains("seven") => Some(7),
    x if x.contains("eight") => Some(8),
    x if x.contains("nine") => Some(9),
    _ => None
  };

  result
}