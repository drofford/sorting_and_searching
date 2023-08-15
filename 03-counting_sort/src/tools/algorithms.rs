use super::customer::Customer;

pub fn counting_sort(inp: &Vec<Customer>, k: i32) -> Vec<Customer> {
    let mut cnt = make_count_vector(k as usize);
    let mut out = make_cust_out_vector(inp.len());

    for i in 0..inp.len() {
        let j = inp[i].num_purchases;
        cnt[j as usize] += 1;
    }

    for i in 1..cnt.len() {
        cnt[i] = cnt[i] + cnt[i-1];
    }

    for i in (0..inp.len()).rev() {
        let j = inp[i].num_purchases;
        cnt[j as usize] = cnt[j as usize] - 1;
        out[cnt[j as usize] as usize] =
            Customer::new(&inp[i].id, inp[i].num_purchases)
    }

    out
}

fn make_count_vector(num_items: usize) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity((num_items + 1) as usize);
    for _ in 0..num_items {
        vec.push(0);
    }
    vec
}

fn make_cust_out_vector(num_items: usize) -> Vec<Customer> {
    let mut vec: Vec<Customer> = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        let cust = Customer::new("", 0);
        vec.push(cust);
    }

    vec
}
