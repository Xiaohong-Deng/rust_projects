use std::env;
use std::collections::LinkedList;

fn main() {
  if env::args().len() < 2 {
    println!("Error: Please provide a number as argument.");
    return;
  }
  // below is an alternative
  //let temp: Vec<String> = env::args().collect();
  let i:u64 = match (env::args().collect::<Vec<String>>())[1].trim().parse() {
    Ok(num) => num,
    Err(_) => return
  };
  println!("Min number that has {} Collatz stpes is {}", i, reverse_min_collatz(i));
}

fn reverse_min_collatz(steps: u64) -> u64 {
  if steps == 0 { return 1; }
  let mut steps_count = steps;
  let mut queue: LinkedList<u64> = LinkedList::new();
  queue.push_back(2);
  steps_count -= 1;
  while steps_count != 0 {
    let mut next_batch: LinkedList<u64> = LinkedList::new();
    while !queue.is_empty() {
      let parent = queue.pop_front().unwrap();
      let child_a = parent * 2;
      let mut child_b = parent - 1;
      if child_b % 3 == 0 { 
        child_b = child_b / 3;
        if (child_b % 2 != 0) & (child_b != 1) {
          //println!("child_b {}: ", child_b);
          next_batch.push_back(child_b);
        }
      }
      //println!("child_a {}: ", child_a);
      next_batch.push_back(child_a);
    }
    queue.append(&mut next_batch);
    steps_count -= 1;
  }
  let mut min = queue.pop_front().unwrap();
  for cand in &queue {
    if *cand < min { min = *cand; }
  }
  return min;
}

#[allow(non_snake_case)]
fn collatz(N: u64) -> u64 {
  if N == 1 { return 0; }
  match N % 2 {
    // an expression, which is not followed by punctuation like semicolon, comma
    // at the the end of a function is treated as a return value, note here
    // no commas for each arm of the match
    0 => { 1 + collatz(N/2) }
    _ => { 1 + collatz(N*3+1) }
  }
}