// Created by Hiago Lucas at 2026/06/13 19:22
// leetgo: 1.4.17
// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut first: i32 = -1;
		let mut last : i32 = -1;
		let mut greatest = i32::MAX;
		let mut on_going = false;

		for i in 1..nums.len() {
			if nums[i - 1] > nums[i] && !on_going {
				if first == -1 {
					first = i as i32 -1;
					greatest = nums[i-1];
					on_going = true;
				} else {
					last = i as i32;
					greatest = nums[i-1];
					on_going = true;
				}
			}

			if on_going && nums[i] >= greatest {
				last = i as i32 - 1;
				on_going = false;
			}
		}

		if first == -1 {
			return 0;
		}

		if last == -1 {
			last = nums.len() as i32 -1;
		}

		dbg!(last, first);

		last - first + 1
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::find_unsorted_subarray(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
