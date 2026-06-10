// Created by Hiago Lucas at 2026/06/09 21:54
// leetgo: 1.4.17
// https://leetcode.com/problems/two-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut solution: Vec<i32> = vec![0, 0];

        for (idx, n) in nums.iter().enumerate() {
            hm.insert(*n, idx);
        }

        for (idx, n) in nums.iter().enumerate() {
            let remain = target - *n;

            if let Some(idx_found) = hm.get(&remain) {
                if *idx_found != idx {
                    solution[0] = idx as i32;
                    solution[1] = *idx_found as i32;

                    return solution;
                }
            }
        }

        return solution;
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::two_sum(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
