use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut sorted_set: BTreeSet<(i32, usize)> = BTreeSet::new();
        let size: usize = temperatures.len();
        let mut response: Vec<i32> = Vec::new();
        response.resize(size, 0);

        for i in 0..size {
            let mut total: i32 = 0;
            for e in sorted_set.iter() {
                if temperatures[i] > e.0 {
                    response[e.1] = (i - e.1) as i32;
                    total += 1;
                }
                else { break };
            }
            for _ in 0..total {
                sorted_set.pop_first();
            }
            sorted_set.insert((temperatures[i], i));
        }
        return response;
    }
}

fn main() {
    let vec1 = vec![73,74,75,71,69,72,76,73];
    let vec2 = vec![30,40,50,60];
    let vec3 = vec![30,60,90];

    let risp1 = vec![1,1,4,2,1,1,0,0];
    let risp2 = vec![1,1,1,0];
    let risp3 = vec![1,1,0];

    assert_eq!(Solution::daily_temperatures(vec1), risp1);   
    assert_eq!(Solution::daily_temperatures(vec2), risp2);
    assert_eq!(Solution::daily_temperatures(vec3), risp3);
}