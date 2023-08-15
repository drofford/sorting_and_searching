mod tools;

use tools::algorithms::counting_sort;
use tools::customer::Customer;
use tools::utils::{print_cust_vec, make_cust_vec, check_sorted, get_i32};

fn main() {
    let num_items = get_i32("Number of items: ");
    let max_num_of_purchases = get_i32("Maximum number of purchases: ");

    let vec: Vec<Customer> = make_cust_vec(num_items, max_num_of_purchases);

    print!("Unsorted vector: ");
    print_cust_vec(&vec, num_items);

    let sorted = counting_sort(&vec, max_num_of_purchases);
    print!("Sorted   vector: ");
    print_cust_vec(&sorted, num_items);

    check_sorted(&sorted);
}
