use std::fs::read_to_string;

fn main() {
  let input: Vec<String> = read_to_string("input.txt")
    .expect("Could not read input")
    .lines()
    .map(String::from)
    .collect();

  let times: Vec<u64> = input[0].split(':')
    .collect::<Vec<&str>>()[1]
    .split_whitespace()
    .map(|str| str.parse::<u64>().unwrap())
    .collect();

  let distances: Vec<u64> = input[1].split(':')
    .collect::<Vec<&str>>()[1]
    .split_whitespace()
    .map(|str| str.parse::<u64>().unwrap())
    .collect();

  let mut time_dist_pairs: Vec<(u64, u64)> = Vec::new();
  let mut monolith_pair: (u64, u64) = (0, 0);
  let mut time_str = String::from("");
  let mut dist_str = String::from("");

  for i in times.into_iter().zip(distances.into_iter()) {
    time_dist_pairs.push(i);
    time_str.push_str(&i.0.to_string());
    dist_str.push_str(&i.1.to_string());
  }

  println!("{dist_str}");

  monolith_pair.0 = time_str.parse::<u64>().unwrap();
  monolith_pair.1 = dist_str.parse::<u64>().unwrap();

  //Part 1
  let mut win_count_list: Vec<u64> = Vec::new();

  for (t, d) in time_dist_pairs {
    let mut win_count = 0;

    for i in 0..t {
      if i * (t - i) > d {
        win_count += 1;
      }
    }

    win_count_list.push(win_count);
  }

  println!("Part 1: {}\n", win_count_list.iter().product::<u64>());

  //Part 2
  let mut wins = 0;
  for i in 0..monolith_pair.0 {
    if i * (monolith_pair.0 - i) > monolith_pair.1 {
      wins += 1;
    }
  }

  println!("Part 2: {wins}");
}