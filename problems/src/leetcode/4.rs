pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total = nums1.len() + nums2.len();
    let mid = total / 2;
    let mut tmp = Vec::with_capacity(mid+1);
    let (mut n, mut m) = (0, 0);
    while tmp.len() <= mid {
        if n < nums1.len() && m < nums2.len() {
            if nums1[n] <= nums2[m] {
                tmp.push(nums1[n]); n += 1;
            } else {
                tmp.push(nums2[m]); m += 1;
            }
        } else if n < nums1.len() {
            tmp.push(nums1[n]); n += 1;
        } else if m < nums2.len() {
            tmp.push(nums2[m]); m += 1;
        }
    }
    if total % 2 == 1 {
        tmp[mid] as f64
    } else {
        (tmp[mid-1] + tmp[mid]) as f64 / 2.0
    }
}

fn main() {
    let a = find_median_sorted_arrays(vec![1,3], vec![2]);
    let b = find_median_sorted_arrays(vec![1,2], vec![3, 4]);
    println!("{a} {b}");
}