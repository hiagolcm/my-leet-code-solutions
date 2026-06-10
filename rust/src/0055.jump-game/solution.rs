// Created by Hiago Lucas at 2026/06/09 22:24
// leetgo: 1.4.17
// https://leetcode.com/problems/jump-game/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut furthest = 0;

		for (idx, n) in nums.iter().enumerate() {
			if furthest < idx {
				return false;
			}

			let current_furthest = *n as usize + idx;

			if current_furthest > furthest {
				furthest = current_furthest;
			}
		}

		furthest >= nums.len() - 1
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: bool = Solution::can_jump(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
