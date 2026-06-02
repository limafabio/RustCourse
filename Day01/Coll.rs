
fn collatz(mut n: i32) -> i32 {
  if n == 1 {
      return n;
   } else if 0 == (n % 2) {
        return collatz((n-1)/2);
   } else {
        return 3*collatz(n-1) + 1;
   }
}

fn main() {
  println!("Length: {}", collatz(3));
}
