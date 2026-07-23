fn main() {
    let s1 = String::from("hello1");
    let addr1 = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one1 = i16::from(true);
    let bigger1 = i32::from(123_i16);
    println!("{s1}, {addr1}, {one1}, {bigger1}");

    let s2: String = "hello2".into();
    let addr2: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one2 : i16 = true.into();
    let bigger2 : i32 = 123_i16.into();
    println!("{s2}, {addr2}, {one2}, {bigger2}");
}
