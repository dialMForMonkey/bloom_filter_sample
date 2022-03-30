mod bloom_filter;

use bloom_filter::{Control, Handler};
use uuid::Uuid;
use rand::Rng;

const MAX: u128 = 1000;

fn main() {

    let mut sub_array:Vec<String> = Vec::with_capacity(100);
    let mut bf =  Control::new( MAX, 2.1);
    let mut rng = rand::thread_rng();
    for _ in 0..MAX {
        let uuid = Uuid::new_v4().to_string();
        bf.add(uuid.as_str());
        if  rng.gen_range(0..2) == 1 && sub_array.len() < 100 {
            sub_array.push(uuid.clone());
        }
    }

    for item in sub_array {
        println!("{} has? {}",item,bf.check(item.as_str()))
    }


}
