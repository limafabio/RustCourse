fn collatz(n: i32) -> i32 {
    if 1 == n {
        1
    } else if 0 == (n % 2) {
        collatz(n/2) + 1
    } else {
        collatz(3*n+1) + 1
    }
}

fn main() {
    println!("Length: {}", collatz(11));
}
