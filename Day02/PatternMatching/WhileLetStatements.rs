fn main() {
    let mut name = String::from("Compreensive Rust 🦀");
    while let Some(c) = name.pop() {
        dbg!(c);
    }
}
