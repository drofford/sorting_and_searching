// // partition.rs

// mod utils;

// // use utils::{
// //     print_vec,
// // };

// /* 
//  The Lomuto algorithm from Wikipedia

// // Divides array into two partitions
// algorithm partition(A, lo, hi) is 
//   pivot := A[hi] // Choose the last element as the pivot

//   // Temporary pivot index
//   i := lo - 1

//   for j := lo to hi - 1 do 
//     // If the current element is less than or equal to the pivot
//     if A[j] <= pivot then 
//       // Move the temporary pivot index forward
//       i := i + 1
//       // Swap the current element with the element at the temporary pivot index
//       swap A[i] with A[j]

//   // Move the pivot element to the correct pivot position (between the smaller and larger elements)
//   i := i + 1
//   swap A[i] with A[hi]
//   return i // the pivot index

// */



// pub fn partition(vec: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {

//     println!("partition: input vector has {} elements", vec.len());
//     println!("partition: input vector is: ");
//     print_vec(&vec, vec.len());

//     let mut pivot = vec[hi as usize - 1];
//     println!("initial pivot = {}", pivot);

//     // let mut i = lo -1;

//     // for j in lo..(hi -1) {

//     // }


//     return 4;
// }