use std::fs::read_to_string;

#[derive(Debug)]
enum Token {
  Empty,
  Symbol,
  Number(u32)
}

fn main() {
  let input: Vec<String> = read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(String::from)
    .collect();

  let schematic: Vec<Vec<Token>> = tokenize(&input);
  let mut visited: Vec<(usize, usize)> = vec![];
  let mut nums: Vec<u32> = vec![];

  for (i, row) in schematic.iter().enumerate() {
    for (j, tok) in row.iter().enumerate() {
      if matches!(tok, Token::Symbol) {
        nums.append(&mut get_adjacent_nums(&schematic, &mut visited, i, j));
      }
    }
  }

  let sum = nums.iter().sum::<u32>();
  println!("{sum}");
}

fn tokenize(input: &Vec<String>) -> Vec<Vec<Token>> {
  let mut schematic: Vec<Vec<Token>> = vec![];

  for line in input {
    let mut tok_line: Vec<Token> = vec![];

    for c in line.chars() {
      match c {
        '.' => tok_line.push(Token::Empty),
        c if c.is_digit(10) => {
          tok_line.push(Token::Number(c.to_digit(10).unwrap()))
        },
        _ => tok_line.push(Token::Symbol)
      };
    }

    schematic.push(tok_line);
  }

  schematic
}

//Assumptions: All words will be left to right. 
//             If two symbols touch the same number, the number is counted once.
fn get_adjacent_nums(schematic: &Vec<Vec<Token>>, visited: &mut Vec<(usize, usize)>, row: usize, col: usize) -> Vec<u32> {
  let mut nums: Vec<u32> = vec![];

  //North West
  if row > 0 && 
    col > 0 && 
    !visited.contains(&(row - 1, col - 1)) {
    if matches!(schematic[row - 1][col - 1], Token::Number(_)) {
      nums.push(get_num(schematic, visited, row - 1, col - 1));
    }
  }

  //North
  if row > 0 && 
    !visited.contains(&(row - 1, col)) {
    if matches!(schematic[row - 1][col], Token::Number(_)) {
      nums.push(get_num(schematic, visited, row - 1, col));
    }
  }

  //North East
  if row > 0 && 
    col < schematic[row].len() - 1 && 
    !visited.contains(&(row - 1, col + 1)) {
    if matches!(schematic[row - 1][col + 1], Token::Number(_)) {
      nums.push(get_num(schematic, visited, row - 1, col + 1));
    }
  }

  //West
  if col > 0 && 
    !visited.contains(&(row, col - 1)) {
    if matches!(schematic[row][col - 1], Token::Number(_)) {
      nums.push(get_num(schematic, visited, row, col - 1));
    }
  }

  //East
  if col < schematic[row].len() - 1 && 
    !visited.contains(&(row, col + 1)) {
    if matches!(schematic[row][col + 1], Token::Number(_)) {
      nums.push(get_num(schematic, visited, row, col + 1));
    }
  }

  //South West
  if row < schematic.len() - 1 && 
    col > 0 && 
    !visited.contains(&(row + 1, col - 1)) {
    if matches!(schematic[row + 1][col - 1], Token::Number(_)) {
      nums.push(get_num(schematic, visited, row + 1, col - 1));
    }
  }

  //South 
  if row < schematic.len() - 1 && 
    !visited.contains(&(row + 1, col)) {
    if matches!(schematic[row + 1][col], Token::Number(_)) {
      nums.push(get_num(schematic, visited, row + 1, col));
    }
  }

  //South East
  if row < schematic.len() - 1 && 
    col < schematic[row].len() - 1 && 
    !visited.contains(&(row + 1, col + 1)) {
    if matches!(schematic[row + 1][col + 1], Token::Number(_)) {
      nums.push(get_num(schematic, visited, row + 1, col + 1));
    }
  }

  nums
}

fn get_num(schematic: &Vec<Vec<Token>>, visited: &mut Vec<(usize, usize)>, row: usize, col: usize) -> u32 {
  let mut digits: Vec<u32> = vec![];
  
  //left
  for i in (0..col).rev() {
    visited.push((row, i));

    match schematic[row][i] {
      Token::Empty => {
        break;
      },
      Token::Symbol => {
        break;
      },
      Token::Number(n) => {
        digits.insert(0, n);
      }
    }
  }

  //right
  for i in col..schematic[row].len() {
    visited.push((row, i));

    match schematic[row][i] {
      Token::Empty => {
        break;
      },
      Token::Symbol => {
        break;
      },
      Token::Number(n) => {
        digits.push(n);
      }
    }
  }

  digits.into_iter()
    .map(|digit| digit.to_string())
    .collect::<String>()
    .parse::<u32>()
    .unwrap_or(0)
}
