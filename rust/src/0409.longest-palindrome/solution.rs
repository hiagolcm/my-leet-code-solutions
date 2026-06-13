// Created by Hiago Lucas at 2026/06/13 18:28
// leetgo: 1.4.17
// https://leetcode.com/problems/longest-palindrome/


use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

use std::collections::HashMap;


// { a:1, b: 1, c:4, d:2 //}
// can_odd = true
// ans = 0
// for each key in h_table
//   if key.val is even {
//     ans += key.val
//   } else {
//     if can_odd { ans += key.val; can_odd = false } else { ans += key.val - 1}
//   }

// ans = 7

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut frequency_map: HashMap<char, i32> = HashMap::new();

		let chars: Vec<char> = s.chars().collect();

		for c in chars {
			frequency_map.entry(c).and_modify(|f| { *f += 1}).or_insert(1);
		}

		let mut ans = 0;
		let mut can_odd = true;

		for (c, f) in &frequency_map {
			if f % 2 == 0 {
				ans += f;
			} else {
				if can_odd {
					ans += f;
					can_odd = false;
				} else {
					ans += f -1;
				}
			}
		}

		ans
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let ans: i32 = Solution::longest_palindrome(s).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
