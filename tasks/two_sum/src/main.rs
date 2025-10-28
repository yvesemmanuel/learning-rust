use std::collections::HashMap;

#[derive(Debug)]
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&complement_index) = map.get(&complement) {
                return vec![complement_index as i32, i as i32];
            }

            map.insert(num, i);
        }

        vec![]
    }
}

fn main() {
    let vec = Vec::from([2, 7, 11, 15]);
    let solution = Solution::two_sum(vec, 9);
    println!("Result: {:?}", solution);
}
