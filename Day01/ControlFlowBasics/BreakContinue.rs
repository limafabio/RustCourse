fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if 0 == i % 2 {
            continue;
        }
        dbg!(i);
    }
}
