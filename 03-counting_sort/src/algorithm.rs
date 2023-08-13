// here's the count sort algorithm in pseudocode from Wikipedia:

// function CountingSort(input, k)

//     count ← array of k + 1 zeros
//     output ← array of same length as input

//     for i = 0 to length(input) - 1 do
//         j = key(input[i])
//         count[j] = count[j] + 1

//     for i = 1 to k do
//         count[i] = count[i] + count[i - 1]

//     for i = length(input) - 1 down to 0 do
//         j = key(input[i])
//         count[j] = count[j] - 1
//         output[count[j]] = input[i]

//     return output



pub fn count_sort(inp: &Vec<i32>, k: i32) -> Vec<i32> {
    let mut cnt = make_new_zero_vec(k as usize);
    let mut out = make_new_zero_vec(inp.len());

    for i in 0..inp.len() {
        let j = inp[i];
        cnt[j as usize] += 1;
    }

    for i in 1..cnt.len() {
        cnt[i] = cnt[i] + cnt[i-1];
    }

    for i in (0..inp.len()).rev() {
        let j = inp[i];
        cnt[j as usize] = cnt[j as usize] - 1;
        out[cnt[j as usize] as usize] = inp[i];
    }

    out
}


fn make_new_zero_vec(num_items: usize) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(0);
    }
    vec
}


// fn partition(array: &mut [i32]) -> usize {
//     let len = array.len();
//     let pivot = array[len - 1];
//     let mut i = 0;
//     let mut j = 0;

//     while j < len - 1 {
//         if array[j] <= pivot {
//             array.swap(i, j);
//             i += 1;
//         }
//         j += 1;
//     }

//     array.swap(i, len - 1);
//     i
// }
