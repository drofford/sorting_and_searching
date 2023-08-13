mod algorithm;
mod utils;

use algorithm::count_sort;
use utils::{check_sorted, make_random_vec, print_vec};

const SIZE: usize = 25;
const MAXVAL: i32 = 50;

fn main() {
    let mut array = make_random_vec(SIZE, MAXVAL);
    print!("initial array of {} values: ", array.len());
    print_vec(&array, SIZE);

    let sorted = count_sort(&mut array, MAXVAL);
    print!("sorted  array of {} values: ", sorted.len());
    print_vec(&sorted, SIZE);

    check_sorted(&sorted);
}
