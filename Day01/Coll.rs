fn collatz(mut n: i32) -> i32 {
    if 1 == n {
        return 1;
    } else if 0 == (n % 2) {
        return collatz(n/2) + 1;
    } else {
        return collatz(3*n+1) + 1 ;
    }
}

fn main() {
    println!("Length: {}", collatz(11));
}
