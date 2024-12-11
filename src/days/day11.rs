use crate::days::PuzzleSolver;
use std::collections::HashMap;

pub struct Day11;

impl PuzzleSolver for Day11 {
  fn solve_part_1(&self, input: &str) -> String {
    let stones: Vec<u64> = input.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut score = 0;
    for stone in stones {
      score += Day11::permute_stone_times(stone, 25, &mut HashMap::new());
    }
    score.to_string()
  }

  fn solve_part_2(&self, input: &str) -> String {
    let stones: Vec<u64> = input.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut score = 0;
    for stone in stones {
      score += Day11::permute_stone_times(stone, 75, &mut HashMap::new());
    }
    score.to_string()
  }
}

impl Day11 {
  fn permute_stone_times(stone: u64, times: usize, memo_table: &mut HashMap<(u64, usize), u64>) -> u64 {
    let memo = memo_table.get(&(stone, times));
    if memo.is_some() {
      return *memo.unwrap()
    }
    let mut result = 1;
    if times > 0 {
      if stone == 0 {
        result =  Day11::permute_stone_times(1, times - 1, memo_table)
      } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
        // Stone has an even number of digits
        let stone_str = stone.to_string();
        let half_len = stone_str.len() / 2;
        let first_half_str = &stone_str[..half_len];
        let second_half_str = &stone_str[half_len..];
        result = Day11::permute_stone_times(first_half_str.parse().unwrap(), times - 1, memo_table) + Day11::permute_stone_times(second_half_str.parse().unwrap(), times - 1, memo_table)
      } else {
        result = Day11::permute_stone_times(stone * 2024, times - 1, memo_table)
      }
    }
    // Remember the result
    memo_table.insert((stone, times), result);
    result
  }
}