// Created by Hiago Lucas at 2026/06/10 10:08
// leetgo: 1.4.17
// https://leetcode.com/problems/largest-number/


// 3 304
// 345 3114

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut cloned_nums = nums.clone();

		cloned_nums.sort_by(|a,b| {
			let a_str = a.to_string();
			let b_str = b.to_string();

			let ab = format!("{a_str}{b_str}");
			let ba = format!("{b_str}{a_str}");

			ba.cmp(&ab)
		});

		if cloned_nums.first() == Some(&0) {
			return String::from("0");
		}

		let mut result:String = String::from("");

		for n in cloned_nums {
			result = format!("{result}{n}");
		}

		result
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: String = Solution::largest_number(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
