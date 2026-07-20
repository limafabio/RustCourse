
fn main() {
    let name = "Löwe 老虎 Léopard Gepardi";
    let mut position: Option<usize> = name.find('é');
    println!("find retornou {position:?}");
    assert_eq!(position.unwrap(), 14);
    position = name.find('Z');
    println!("find retornou {position:?}");
    assert_eq!(position.expect("Caratere não encontrado"),0);
}
