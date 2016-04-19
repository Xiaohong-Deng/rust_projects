use std::env;

fn main() {
  if env::args().len() < 2 {
    println!("Error: Please provide a number as argument.");
    return;
  }
  // below is an alternative
  //let temp: Vec<String> = env::args().collect();
  let i:u32 = match (env::args().collect::<Vec<String>>())[1].trim().parse() {
    Ok(num) => num,
    Err(_) => return
  };
  println!("{} has {} Collatz steps", i, collatz(i));
}

#[allow(non_snake_case)]
fn collatz(N: u32) -> u32 {
  if N == 1 { return 0; }
  match N % 2 {
    0 => { 1 + collatz(N/2) }
    _ => { 1 + collatz(N*3+1) }
  }
}