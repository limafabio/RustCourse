fn main() {
    let x = 101;
    if 0 == x {
        println!("zero");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }

    let y = 1;
    let size = if y < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}
