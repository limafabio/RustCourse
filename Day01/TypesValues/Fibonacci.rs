fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 2;
    println!("fib({n}) = {}", fib(n));
    let n = 3;
    println!("fib({n}) = {}", fib(n));
    let n = 4;
    println!("fib({n}) = {}", fib(n));
    let n = 5;
    println!("fib({n}) = {}", fib(n));
    let n = 6;
    println!("fib({n}) = {}", fib(n));
    let n = 57;
    println!("fib({n}) = {}", fib(n));
    let n = 32;
    println!("fib({n}) = {}", fib(n));
}
