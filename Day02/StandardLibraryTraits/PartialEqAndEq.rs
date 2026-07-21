
struct Key {
    id: u32,
    metadata: Option<String>,
}

impl PartialEq for Key {
   fn eq(&self, other: &Self) -> bool {
       self.id == other.id
   }
}

fn main() {
  let k1 = Key{ id: 32, metadata: Some(String::from("k1")) };
  let k2 = Key{ id: 31, metadata: Some(String::from("k2")) };
  
  if k1.eq(&k2) {
    println!("Are Equals");
  } else {
    println!("Not Equals");
  }

}
