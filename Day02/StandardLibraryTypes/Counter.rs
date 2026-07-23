
use std::collections::HashMap;
use std::hash::Hash;

struct Counter<T> {
		values: HashMap< T, u64 >,
}

impl<T: Eq + Hash > Counter<T> {
		fn new() -> Self {
				Counter {
						values: HashMap::new(),
				}
		}

    fn count (&mut self, value: T ) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

		fn times_seen(&self, value: T ) -> u64 {
				self.values.get(&value).copied().unwrap_or_default()
		}
}

fn main() {
		let mut ctr = Counter::new();
		ctr.count(13);
		ctr.count(14);
		ctr.count(16);
		ctr.count(14);
		ctr.count(14);
		ctr.count(11);

		for i in 10..20 {
				println!("saw {} values equal to {}", ctr.times_seen(i), i);
		}

		let mut strctr: Counter<String> = Counter::new();
		strctr.count(String::from("apple"));
		strctr.count(String::from("orange"));
 	  strctr.count(String::from("apple"));
		println!("got {} apple", strctr.times_seen(String::from("apple")));
}










