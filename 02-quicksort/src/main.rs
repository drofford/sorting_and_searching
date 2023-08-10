mod partition;
mod utils;

use partition::{
    partition,
};

use utils::{
    print_vec,
};

fn main() {
    println!("Hello, world!");
    let r = partition();
    println!("result = {}", r);

    vector_play();
}

const SIZE: i32 = 10;

fn vector_play() {
    let mut vec: Vec<i32> = Vec::with_capacity(SIZE as usize);

    for i in 0..SIZE {
        vec.push(i);
    }

    print_vec(&vec, 2 * SIZE);
}

