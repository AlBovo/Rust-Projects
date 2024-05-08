struct Solution;

// https://leetcode.com/problems/hamming-distance/
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let xored_num = x ^ y;
        return xored_num.count_ones() as i32;
    }
}

fn main(){
    assert_eq!(Solution::hamming_distance(1, 4), 2);
    assert_eq!(Solution::hamming_distance(3, 1), 1);
}