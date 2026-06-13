// Created by Hiago Lucas at 2026/06/13 18:58
// leetgo: 1.4.17
// https://leetcode.com/problems/array-partition/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut cloned_nums = nums.clone();
		cloned_nums.sort();

		let mut ans = 0;

		for (idx, n) in cloned_nums.iter().enumerate() {
			if idx % 2 == 0 {
				ans += *n;
			}
		}

		ans
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::array_pair_sum(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
