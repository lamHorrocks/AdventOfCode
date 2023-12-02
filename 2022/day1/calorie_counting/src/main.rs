use std::fs;

fn main() {
  let file_content = fs::read_to_string("input.txt")
    .expect("Could not read file");

  let split_content = file_content.split("\n");

  let mut calorie_counts: Vec<i32> = Vec::new();
  let mut curr_sum = 0;
  for str in split_content {
    match str.is_empty() {
      true => {
        calorie_counts.push(curr_sum);
        curr_sum = 0;
      },
      false => {
        curr_sum += str.parse::<i32>().expect("Could not parse to int");
      }
    }
  }

  let result_vec = get_max_three(&calorie_counts);

  println!("The three maximum values are: ");

  match result_vec {
    Some(vec) => {
      for count in &vec {
        println!("{count}");
      }

      let sum: i32 = vec.iter().sum();
      println!("\nThe sum is: {sum}");
    }
    None => {
      println!("result vector is empty")
    }
  }
}

fn get_max_three(vec: &Vec<i32>) -> Option<Vec<i32>> {
  if vec.len() < 3 {
    return None;
  }

  let mut cloned_vec = vec.clone();
  cloned_vec.sort();
  cloned_vec = vec![
                    cloned_vec[cloned_vec.len() - 1], 
                    cloned_vec[cloned_vec.len() - 2], 
                    cloned_vec[cloned_vec.len() - 3]
                  ];

  Some(cloned_vec)
}
