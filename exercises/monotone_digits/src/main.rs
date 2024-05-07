struct Solution;

// https://leetcode.com/problems/monotone-increasing-digits
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        if n < 10 {
            return n;
        }
        
        let size = n.ilog10();
        let mut num: i32 = 0;
        let mut last: i32 = 0;

        for i in 0..=size {
            let digit = (n / i32::pow(10, size-i)) % 10;
            
            if num + digit.to_string().repeat((size-i+1) as usize).parse::<i32>().unwrap() <= n && last <= digit {
                num += digit * i32::pow(10, size-i);
                last = digit;
            }
            else {
                num += (digit - 1) * i32::pow(10, size-i);
                num += "9".repeat((size-i) as usize).parse::<i32>().unwrap();
                break;
            }
        }

        return num;
    }
}

fn main() {
    assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    assert_eq!(Solution::monotone_increasing_digits(10), 9);
    assert_eq!(Solution::monotone_increasing_digits(332), 299);
}