use std::time::{SystemTime, UNIX_EPOCH};

const SIZE: usize = 25;

fn main() {
    let mut array = make_random_vec(SIZE, 100);
    print!("initial array of {} values: ", array.len());
    print_vec(&array, SIZE);

    quicksort(&mut array);
    print!("initial array of {} values: ", array.len());
    print_vec(&array, SIZE);

    check_sorted(&array);
}

fn quicksort(array: &mut [i32]) {
    if !array.is_empty() {
        let pivot_idx = partition(array);
        let len = array.len();

        quicksort(&mut array[0..pivot_idx]);
        quicksort(&mut array[pivot_idx + 1..len]);
    }
}

fn partition(array: &mut [i32]) -> usize {
    let len = array.len();
    let pivot = array[len - 1];
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if array[j] <= pivot {
            array.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    array.swap(i, len - 1);
    i
}

fn print_vec(vec: &Vec<i32>, num_items: usize) {
    let mut max = vec.len();
    if max > num_items
    /* as usize */
    {
        max = num_items /* as usize */;
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

fn check_sorted(vec: &Vec<i32>) {
    for i in 1usize..vec.len() {
        if vec[i] < vec[i - 1] {
            println!("The vector is NOT sorted!");
        }
    }
    println!("The vector is sorted!");
}

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
fn make_random_vec(num_items: usize, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    return vec;
}
