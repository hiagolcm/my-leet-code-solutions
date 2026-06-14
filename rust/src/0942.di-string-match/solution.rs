// Created by Hiago Lucas at 2026/06/13 20:32
// leetgo: 1.4.17
// https://leetcode.com/problems/di-string-match/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut list: Vec<i32> = vec![];
		let chars: Vec<char> = s.chars().collect();
		let mut min = 0;
		let mut max = s.len() as i32;

		if chars[0] == 'D' {
			list.push(max as i32);
			max -= 1;
		} else {
			list.push(min);
			min +=1;
		}

		let mut idx = 0;

		while idx < chars.len() {
			let c = chars[idx];

			if c == 'D' {
				while idx + 1 < chars.len() && chars[idx+1] == 'D' {
					list.push(max);
					max -= 1;
					idx +=1;
				}

				list.push(min);
				min +=1;
			} else {
				while idx + 1 < chars.len() && chars[idx+1] == 'I' {
					list.push(min);
					min += 1;
					idx +=1;
				}

				list.push(max);
				max -= 1;
			}

			idx +=1;
		}

		list
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::di_string_match(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
