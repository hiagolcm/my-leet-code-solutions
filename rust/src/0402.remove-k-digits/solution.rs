// Created by Hiago Lucas at 2026/06/13 16:59
// leetgo: 1.4.17
// https://leetcode.com/problems/remove-k-digits/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// 1 2 3

// 1432219
// n = 2
// total = 2

// 1 2 1 9

// n = 3
// total = 3
// 10200

// stack = [0200]




// if k >= str.len {return 0} 
// stack = []
// for idx, c in str:
//   if stack.length <= str.length - idx {
//     stack.push(remaining_letters)
//     return stack
//	 }
//
//   if c > stack.last
//     stack.push(c)
//   else 
//      stack.pop()
//	    stack.push(c)
//  return stack

// 1432219
// k = 
// [1, 2, 1, 9]
// [1, 2, 1, 9]


// @lc code=begin

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        if k >= num.len() as i32 {
			return String::from("0");
		}

		let chars: Vec<char> = num.chars().collect();
		let mut stack: Vec<char> = vec![];
		let mut swaps = k;


		for (idx, c) in chars.iter().enumerate() {
			if swaps <= 0 {
				for i in idx..num.len() {
					stack.push(chars[i]);
				}

				break;
			}

			if stack.len() >= chars.len() - k as usize && *c >= stack[stack.len() - 1] {
				swaps-=1;
				continue;
			} else if stack.len() == 0 || *c >= stack[stack.len() - 1] {
				stack.push(*c);
			} else {
				while stack.len() > 0 && swaps > 0 && stack[stack.len() - 1] >*c {
					swaps -= 1;
					stack.pop();
				}

				stack.push(*c);
			}
		}


		let mut answer = String::from("");

		let mut possibly_has_trailing_zero = true;
		for c in stack {
			if c == '0' &&  possibly_has_trailing_zero{
				continue
			} else {
				possibly_has_trailing_zero = false;
			}


			answer = format!("{answer}{c}");
		}

		if answer.len() == 0 {
			return String::from("0");
		}

		answer
    }
}

// @lc code=end

fn main() -> Result<()> {
	let num: String = deserialize(&read_line()?)?;
	let k: i32 = deserialize(&read_line()?)?;
	let ans: String = Solution::remove_kdigits(num, k).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
