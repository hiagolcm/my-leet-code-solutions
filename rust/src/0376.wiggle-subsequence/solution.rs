// Created by Hiago Lucas at 2026/06/12 13:17
// leetgo: 1.4.17
// https://leetcode.com/problems/wiggle-subsequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// [1,17,5,10,13,15,10,5,16,8]
// [1, 17, 5, 15, 5, 16, 8]

// @lc code=begin

#[derive(PartialEq)]
enum State {
    Greater,
    Lesser,
    Undefined,
}

//

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut sequence_length = 1;
        let mut last_status = State::Undefined;
        let mut last_num = nums[0];

        for i in 1..nums.len() {
            let diff = nums[i] - last_num;

            let status = if diff > 0 {
                State::Greater
            } else {
                State::Lesser
            };

            if diff == 0 {
                continue;
            }

            if last_status != status {
                last_status = status;
                sequence_length += 1;
                last_num = nums[i];
            } else {
                if status == State::Greater && nums[i] > last_num
                    || status == State::Lesser && nums[i] < last_num
                {
                    last_num = nums[i];
                }
            }
        }

        sequence_length
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::wiggle_max_length(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
