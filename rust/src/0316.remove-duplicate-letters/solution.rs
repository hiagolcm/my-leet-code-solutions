// Created by Hiago Lucas at 2026/06/10 11:34
// leetgo: 1.4.17
// https://leetcode.com/problems/remove-duplicate-letters/

//

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut seen_letters: HashMap<char, usize> = HashMap::new();
        let mut result = String::from("");

        for (current_idx, c) in s.chars().enumerate() {
            match seen_letters.get(&c) {
                None => {
                    result = format!("{result}{c}");
                    seen_letters.insert(c, result.len() - 1);
                }
                Some(_asdf) => {
                    let mut should_swap = false;

                    let i = result.find(c);

                    if let Some(idx) = i {
                        let result_chars: Vec<char> = result.chars().collect();
                        let mut j = idx + 1;
                        while j < result.len() {
                            if result_chars[j] < c {
                                should_swap = true;
								break;
                            } else {
								let last_l = s.rfind(result_chars[j]); 

								if let Some(other_idx) = last_l {
									if other_idx <= current_idx {
										break;
									}
								}
							}

                            j += 1;
                        }

                        if should_swap {
                            let before = &result[..idx];
                            let after = &result[idx + 1..];
                            result = format!("{before}{after}{c}");
                            seen_letters.insert(c, result.len() - 1);
                        }
                    }
                }
            }
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: String = Solution::remove_duplicate_letters(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
