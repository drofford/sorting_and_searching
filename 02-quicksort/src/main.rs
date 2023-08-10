mod partition;
mod utils;

// use partition::{
//     partition,
// };

use utils::{
    make_random_vec,
    print_vec,
    partition,
};

const SIZE: i32 = 25;

fn main() {
    println!("main: Hello, world!");

    let mut values = make_random_vec(SIZE, 10);
    print!("main: initial vector of {} elements: ", values.len());
    print_vec(&values, SIZE);

    let r = partition(&mut values, 0, SIZE - 1 as i32);
    println!("main: result = {}", r);

}
