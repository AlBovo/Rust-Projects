struct Solution;

// https://leetcode.com/problems/toeplitz-matrix/
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let mut x = 0;
        let mut y = 0;
        
        for i in 0..matrix.len() {
            x = i;
            y = 0;
            loop {
                let first_val = matrix[x][y];

                if x+1 == matrix.len() || y+1 == matrix[0].len() {
                    break;
                }

                let second_val = matrix[x+1][y+1];

                if first_val != second_val {
                    return false;
                }

                x += 1;
                y += 1;
            }
        }

        for i in 0..matrix[0].len() {
            y = i;
            x = 0;
            loop {
                let first_val = matrix[x][y];

                if x+1 == matrix.len() || y+1 == matrix[0].len() {
                    break;
                }

                let second_val = matrix[x+1][y+1];

                if first_val != second_val {
                    return false;
                }

                x += 1;
                y += 1;
            }
        }

        return true;
    }
}

fn main() {
    assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1,2,3,4], vec![5,1,2,3], vec![9,5,1,2]]), true);
    assert_eq!(Solution::is_toeplitz_matrix(vec![vec![1,2], vec![2,2]]), false);
}