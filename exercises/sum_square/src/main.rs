use std::collections::HashSet;

struct Solution;

// https://leetcode.com/problems/sum-of-square-numbers/
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut squares: HashSet<usize> = HashSet::new();
        
        for i in 0..=(c as usize) {
            if i * i > c as usize {
                return false;
            }

            squares.insert(i * i);

            if squares.contains(&((c as usize) - i * i)) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    assert_eq!(Solution::judge_square_sum(5), true);
    assert_eq!(Solution::judge_square_sum(3), false);
}