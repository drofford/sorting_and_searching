mod partition;
mod utils;

use partition::{
    partition,
};

use utils::{
    make_random_vec,
    print_vec,
};

const SIZE: i32 = 100;

fn main() {
    println!("Hello, world!");
    let r = partition();
    println!("result = {}", r);

    let mut values = make_random_vec(SIZE, 10);

    print_vec(&values, SIZE)
}
