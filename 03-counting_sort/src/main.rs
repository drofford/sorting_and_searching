mod algorithm;
mod utils;

// use utils::check_sorted;
// use utils::make_random_vec;
// use utils::print_vec;
// use utils::Prng;

use algorithm::quicksort;
use utils::{check_sorted, make_random_vec, print_vec};

const SIZE: usize = 25;
const MAXVAL: i32 = 100;

fn main() {
    let mut array = make_random_vec(SIZE, MAXVAL);
    print!("initial array of {} values: ", array.len());
    print_vec(&array, SIZE);

    quicksort(&mut array);
    print!("initial array of {} values: ", array.len());
    print_vec(&array, SIZE);

    check_sorted(&array);
}

// fn quicksort(array: &mut [i32]) {
//     if !array.is_empty() {
//         let pivot_idx = partition(array);
//         let len = array.len();
//
//         quicksort(&mut array[0..pivot_idx]);
//         quicksort(&mut array[pivot_idx + 1..len]);
//     }
// }
//
// fn partition(array: &mut [i32]) -> usize {
//     let len = array.len();
//     let pivot = array[len - 1];
//     let mut i = 0;
//     let mut j = 0;
//
//     while j < len - 1 {
//         if array[j] <= pivot {
//             array.swap(i, j);
//             i += 1;
//         }
//         j += 1;
//     }
//
//     array.swap(i, len - 1);
//     i
// }
