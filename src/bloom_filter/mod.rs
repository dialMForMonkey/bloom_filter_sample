use std::f64::consts::E;
use std::hash::Hasher;

pub struct Control {
    hash_count: u128,
    bit_array: Vec<bool>
}

pub trait Handler {
    fn new(total_items: u128, probability_false_positive: f64) -> Self;
    fn add(&mut self, value: &str);
    fn check(&self, value: &str) -> bool;
}

fn get_size(total_items: u128,  probability_false_positive: f64) -> usize {
    let c = 0.4804530139182014;
    let a = probability_false_positive.log(E)/c;
    if  a > 0f64 {
        return (total_items * (a as u128)) as usize;
    }
    return (total_items * ((a as i128  *-1) as u128)) as usize;
}

fn get_hash_count(length_bit_array :usize , total_items: u128) -> u128 {
    length_bit_array as u128/total_items
}

impl Handler for Control {
    fn new(total_items: u128, probability_false_positive: f64) -> Self{
        let length_bit_array = get_size(total_items, probability_false_positive);
        let hash_count = get_hash_count(length_bit_array, total_items);
        Control{
            hash_count,
            bit_array: vec![false; length_bit_array]
        }
    }
    fn add(&mut self, value: &str) {
        for i in 0..self.hash_count {
            let mut hash = twox_hash::Xxh3Hash64::default();
            hash.write(format!("{}{}", value, i).as_bytes());
            let pos =  (hash.finish() as usize) % self.bit_array.len();
            self.bit_array[pos] = true;
        }
    }

    fn check(&self, value: &str) ->bool {
        for i in 0..self.hash_count {
            let mut hash = twox_hash::Xxh3Hash64::default();
            hash.write(format!("{}{}", value, i).as_bytes());
            let pos =  (hash.finish() as usize) % self.bit_array.len();
            if self.bit_array[pos] == false {
                return false ;
            }
        }
        return true;
    }
}
