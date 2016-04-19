fn main() {
  let x = (49, true);
  match x {
    (x, y) if y == true => match x {
      20...26 => println!("True and in range!"),
      40...49 => println!("In another range!"),
      _ => println!("True and out of range!")
      },
    (x, _) => match x {
      40...49 => println!("In another range!"),
      _ => println!("None of the above!")
      }
  }
}