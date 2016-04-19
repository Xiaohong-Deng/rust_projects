fn main() {
  // range is outdated
  for x in 1..101 {
    if x % 3 == 0 {
      if x % 5 == 0 {
        println!("FizzBuzz");
      } else {
        println!("Fizz");
      }
    } else if x % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", x);
    }
  }
}