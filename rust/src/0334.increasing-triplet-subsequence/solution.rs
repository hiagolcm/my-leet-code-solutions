// Created by Hiago Lucas at 2026/06/12 12:20
// leetgo: 1.4.17
// https://leetcode.com/problems/increasing-triplet-subsequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// [20, 100, 10, 12, 5, 13]
// [20,100,]

// [20, 100, 10, 105]

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;
        for n in nums {
            if n <= first {
                first = n;
            } else if n <= second {
                second = n;
            } else {
                return true; // n > second > first
            }
        }
        false
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: bool = Solution::increasing_triplet(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
