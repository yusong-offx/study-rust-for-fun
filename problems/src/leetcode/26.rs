// use std::collections::BTreeSet;

// impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         let mut set = BTreeSet::new();
//         for &n in nums.iter() {
//             set.insert(n);
//         }
//         for (i, &n) in set.iter().enumerate() {
//             nums[i] = n;
//         }
//         set.len() as i32
//     }
// }

// impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         let mut loc = 0;
//         (1..nums.len()).for_each(|i| {
//             if nums[loc] != nums[i] {
//                 loc += 1;
//                 nums[loc] = nums[i];
//             } 
//         });
//         (loc + 1) as i32
//     }
// }

use std::time::{Instant};
// use std::thread;
// use std::time::Duration;

fn my_fun() {
    let mut p = 0;
    for i in 1..1000 {
        p = i;
    }
    // (1..1000).for_each(|i| {
    //     p = i;
    // });
    println!("{p}");
}

fn main() {
    let current = Instant::now();
    
    my_fun();
    
    let duration = current.elapsed();
    
    println!("Time elapsed in MyFun() is: {:?}", duration);
}