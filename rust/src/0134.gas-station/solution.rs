// Created by Hiago Lucas at 2026/06/10 06:27
// leetgo: 1.4.17
// https://leetcode.com/problems/gas-station/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut greedy_idx: usize = 0;
        let mut greedy_gas = 0;

        for i in 0..gas.len() {
            let total_gas = gas[i] - cost[i];
            greedy_gas = greedy_gas + total_gas;

            if total_gas > greedy_gas {
                greedy_gas = total_gas;
                greedy_idx = i;
            }
        }

        if greedy_gas < 0 {
            return -1;
        }

        let mut idx = (greedy_idx + 1) % gas.len();
        let mut current_gas = gas[greedy_idx] - cost[greedy_idx];

        while idx != greedy_idx {
            current_gas = current_gas + gas[idx] - cost[idx];

            if current_gas < 0 {
                return -1;
            }

            idx = (idx + 1) % gas.len();
        }

        return greedy_idx as i32;
    }
}

// @lc code=end

fn main() -> Result<()> {
    let gas: Vec<i32> = deserialize(&read_line()?)?;
    let cost: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::can_complete_circuit(gas, cost).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
