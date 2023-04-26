struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let mut new_nums = Vec::with_capacity(m+n);
    
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            if nums1[i] > nums2[j] {
                new_nums.push(nums2[j]);
                j += 1;
            } else {
                new_nums.push(nums1[i]);
                i += 1;
            }
        }

        while i < m {
            new_nums.push(nums1[i]);
            i += 1;
        }

        while j < n {
            new_nums.push(nums2[j]);
            j += 1;
        }
        *nums1 = new_nums;
    }
}

fn main() {
    let mut num = vec![-1,0,0,3,3,3,0,0,0];
    Solution::merge(
        &mut num, 6, 
        &mut vec![1,2,2], 3
    );
    assert_eq!(
        num,
        vec![-1,0,0,1,2,2,3,3,3]
    );
}