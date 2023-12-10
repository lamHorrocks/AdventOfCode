use std::fs::read_to_string;

fn main() {
  let input: Vec<String> = read_to_string("input.txt")
    .expect("Could not read input")
    .lines()
    .map(String::from)
    .collect();

  let times: Vec<u32> = input[0].split(':')
    .collect::<Vec<&str>>()[1]
    .split_whitespace()
    .map(|str| str.parse::<u32>().unwrap())
    .collect();

  let distances: Vec<u32> = input[1].split(':')
    .collect::<Vec<&str>>()[1]
    .split_whitespace()
    .map(|str| str.parse::<u32>().unwrap())
    .collect();

  let mut time_dist_pairs: Vec<(u32, u32)> = Vec::new();

  for i in times.into_iter().zip(distances.into_iter()) {
    time_dist_pairs.push(i);
  }

  let mut win_count_list: Vec<u32> = Vec::new();

  for (t, d) in time_dist_pairs {
    let mut win_count = 0;

    for i in 0..t {
      if i * (t - i) > d {
        win_count += 1;
      }
    }

    win_count_list.push(win_count);
  }

  println!("{}", win_count_list.iter().product::<u32>());
}
