fn main() {
    let mut s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    s1 = "Space";
    println!("s1: {s1}");

    println!("s2: {s2}");

    let s3: &str = &s2[2..9];
    println!("s3: {s3}");
    test();
}

fn test() {
    println!("{:?}", b"abc");
    println!("{:?}",&[97, 98, 99]);
}
