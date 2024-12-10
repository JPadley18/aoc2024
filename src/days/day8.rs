use crate::days::PuzzleSolver;
use std::collections::HashMap;

pub struct Day8;

impl PuzzleSolver for Day8 {
  fn solve_part_1(&self, input: &str) -> String {
    let mut antinodes: Vec<(usize, usize)> = Vec::new();
    let (node_sets, map_bounds) = Day8::parse_node_lists(input);
    for node_set in node_sets.values() {
      // Iterate over each pair and add the antinodes to the resulting list
      for (i, node) in node_set.iter().enumerate() {
        // Create an iterator that skips the one being checked
        let skip_current = node_set.iter().take(i).chain(node_set.iter().skip(i + 1));
        for other_node in skip_current {
          // Add the calculated antinode if it doesn't already exist
          let antinode = (node.0 as i32 + ((other_node.0 as i32 - node.0 as i32) * 2), node.1 as i32 + ((other_node.1 as i32 - node.1 as i32) * 2));
          if antinode.0 <= map_bounds.0 as i32 && antinode.1 <= map_bounds.1 as i32 && antinode.0 >= 0 && antinode.1 >= 0 {
            let antinode_as_usize = (antinode.0 as usize, antinode.1 as usize);
            if !antinodes.contains(&antinode_as_usize) {
              dbg!(node);
              dbg!(other_node);
              antinodes.push(dbg!(antinode_as_usize));
            }
          }
        }
      }
    }
    antinodes.len().to_string()
  }
}

impl Day8 {
  fn parse_node_lists(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, (usize, usize)) {
    let mut node_lists: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
      for (x, char) in line.chars().enumerate() {
        if char != '.' {
          // Insert the node into the HashMap of its 'frequency'
          node_lists.entry(char).or_insert_with(|| Vec::new()).push((x, y));
        }
      }
    }
    let map_bounds = (node_lists.values().next().unwrap().len(), node_lists.values().len());
    (node_lists, map_bounds)
  }
}