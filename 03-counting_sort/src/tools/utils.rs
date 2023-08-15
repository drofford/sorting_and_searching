use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use super::customer::Customer;

pub fn print_cust_vec(vec: &Vec<Customer>, num_items: i32) {
    let mut max: i32 = vec.len() as i32;
    if max > num_items {
        max = num_items;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0 {
        string.push_str(&vec[0].to_string());
    }

    for i in 1..max {
        string.push_str(" ");
        string.push_str(&vec[i as usize].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

pub fn make_cust_vec(num_items: i32, max_num_purchases: i32) -> Vec<Customer> {
    let mut prng = Prng::new();

    let mut vec: Vec<Customer> = Vec::with_capacity(num_items as usize);
    for n in 0..num_items {
        let name = "C".to_string() + &n.to_string();
        let num_purchases = prng.next_i32(0, max_num_purchases);
        let cust = Customer::new(&name, num_purchases);
        vec.push(cust);
    }

    vec
}

pub fn check_sorted(vec: &Vec<Customer>) {
    for i in 1usize..vec.len() {
        if vec[i].num_purchases < vec[i - 1].num_purchases {
            println!("The vector is NOT sorted!");
            return;
        }
    }
    println!("The vector is sorted!");
}

struct Prng {
    seed: u32,
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}

// Prompt the user for an i32.
pub fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>().expect("Error parsing integer");
}
