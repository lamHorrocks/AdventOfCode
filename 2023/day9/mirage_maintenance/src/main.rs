use std::fs::read_to_string;

fn main() {
  let input: Vec<String> = read_to_string("input.txt")
    .expect("Could not read input file")
    .lines()
    .map(String::from)
    .collect();

  let mut sequences: Vec<Vec<i32>> = vec![];

  for line in input {
    sequences.push(line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect());
  }
  
  let mut sum = 0;

  for sequence in sequences {
    sum += get_sequence_value(&sequence);
  }

  println!("{sum}");
}

//Part 1
fn get_sequence_value(sequence: &Vec<i32>) -> i32 {
  let derived = derive_sequences(&sequence);
  let mut sum = 0;

  for sequence in derived.iter().rev() {
    sum += sequence.last().unwrap()
  }

  sum
}

fn derive_sequences(sequence: &Vec<i32>) -> Vec<Vec<i32>> {
  let mut derived: Vec<Vec<i32>> = vec![];
  let mut count = 0;

  derived.push(sequence.clone());

  while !derived[count].iter().all(|&x| x == 0) {
    derived.push(get_next_sequence(&derived[count]));
    count += 1;
  }

  derived
}

fn get_next_sequence(sequence: &Vec<i32>) -> Vec<i32> {
  let mut next_sequence = vec![];

  for i in 0..sequence.len() - 1 {
    next_sequence.push(sequence[i + 1] - sequence[i]);
  }

  next_sequence
}
