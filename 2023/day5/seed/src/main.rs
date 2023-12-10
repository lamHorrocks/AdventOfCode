use std::fs::read_to_string;

fn main() {
  let input: Vec<String> = read_to_string("input.txt")
    .expect("Could not read file")
    .lines()
    .map(String::from)
    .collect();

  let mut seeds: Vec<u128> = vec![];
  let mut all_maps: Vec<Vec<(u128, u128, u128)>> = vec![];
  let mut map_list: Vec<(u128, u128, u128)> = vec![];

  //This is hideous 
  for line in input {
    if line.is_empty() {
      continue;
    } else if line.contains("seeds:") {
      seeds = line.split(':')
        .last()
        .unwrap()
        .to_string()
        .split_whitespace()
        .map(|str| str.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    } else if line.contains("map") {
      if !map_list.is_empty() {
        map_list.sort_by_key(|map| map.0);
        all_maps.push(std::mem::take(&mut map_list));
        map_list.clear();
      }
    } else {
      let nums: Vec<u128> = line.split_whitespace().map(|str| str.parse::<u128>().unwrap()).collect();
      if nums.len() > 2 {
        map_list.push((nums[0], nums[1], nums[2]));
      }
    }
  }

  if !map_list.is_empty() {
    all_maps.push(std::mem::take(&mut map_list));
  }

  //Part 1
  let mut items = seeds.clone();

  for maps in &all_maps {
    items = sieve(&items, &maps);
  }

  println!("{}", items.iter().min().unwrap());
}

fn sieve(items: &[u128], ranges: &[(u128, u128, u128)]) -> Vec<u128> {
  let mut result: Vec<u128> = Vec::new();

  for item in items {
    let mut mapped_item = item.clone();

    for range in ranges {
      if let Some(num) = transform(item, range) {
        mapped_item = num;
      }
    }

    result.push(mapped_item);
  }

  result
}

fn transform(num: &u128, map: &(u128, u128, u128)) -> Option<u128> {
  let source_range_start = map.1;
  let source_range_end = source_range_start + map.2;

  if num >= &source_range_start && num < &source_range_end {
    let offset = num - source_range_start;
    return Some(map.0 + offset);
  }

  None
}