// utils.rs

use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

pub fn check_sorted(vec: &Vec<i32>) {
    for i in 1usize..vec.len() {
        if vec[i] < vec[i-1] {
            println!("The vector is NOT sorted!");
        }
    }
    println!("The vector is sorted!");
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

// ************
// *** Prng ***
// ************
struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
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

// Make a vector of random i32 values in the range [0 and max).
pub fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    return vec;
}

/* 
 The Lomuto algorithm from Wikipedia

// Divides array into two partitions
algorithm partition(A, lo, hi) is 
  pivot := A[hi] // Choose the last element as the pivot

  // Temporary pivot index
  i := lo - 1

  for j := lo to hi - 1 do 
    // If the current element is less than or equal to the pivot
    if A[j] <= pivot then 
      // Move the temporary pivot index forward
      i := i + 1
      // Swap the current element with the element at the temporary pivot index
      swap A[i] with A[j]

  // Move the pivot element to the correct pivot position (between the smaller and larger elements)
  i := i + 1
  swap A[i] with A[hi]
  return i // the pivot index

*/



pub fn partition(vec: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {
    print!("partition: input vector has {} elements: ", vec.len());
    print_vec(&vec, vec.len() as i32);

    let pivot = vec[hi as usize];
    println!("partition: initial pivot = {}", pivot);

    let mut i = lo -1;

    for j in lo..(hi - 1) {
        print!("partition: pivot={}, lo={}, hi={}, i={}, j={}", 
            pivot, lo, hi, i, j);
        if i >= 0 { print!(", vec[i] = {}", vec[i as usize]); }
        if j >= 0 { print!(", vec[j] = {}", vec[j as usize]); }
        println!("");
        // println!("partition: pivot={}, lo={}, hi={}, i={}, j={}, vec[i]={}, vec[j]={}", 
        //     pivot, lo, hi, i, j, vec[i as usize], vec[j as usize]);
        // println!("partition: pivot={}, lo={}, hi={}, i={}, j={}, vec[i]={}, vec[j]={}", 
        //     pivot, lo, hi, i, j, vec[i as usize], vec[j as usize]);
        if vec[j as usize] <= pivot {
            i += 1;
            vec.swap(i.try_into().unwrap(), j.try_into().unwrap());
        }
    }
    i += 1;
    vec.swap(i.try_into().unwrap(), hi.try_into().unwrap());
    return i;
}