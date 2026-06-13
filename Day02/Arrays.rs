fn main() {
  let mut a: [i8; 5] = [5, 4, 3, 2, 1];
  a[2] = 0;
  let b: [i8; 5] =  [0; 5];
  println!("a: {a:?}");
  println!("b: {b:#?}");
}
