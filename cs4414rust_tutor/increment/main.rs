fn main() {
 let p = vec![1, 2, 3];
 let q = increment(p);
 for e in &q {
  println!("{}", e);
 }

 let mut p = vec![1, 2, 3];
 increment_mut(&mut p);
 for e in &p {
  println!("{}", e);
 }
}

fn increment_mut(mv: &mut Vec<i32>) {
  for e in mv {
    *e = *e + 1;
  }
}

fn increment(v: Vec<i32>) -> Vec<i32> {
  let mut rv = Vec::new();
  for e in &v {
    rv.push(e + 1);
  }
  rv
}