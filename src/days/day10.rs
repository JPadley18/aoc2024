use crate::days::PuzzleSolver;

pub struct Day10;

impl PuzzleSolver for Day10 {
  fn solve_part_1(&self, input: &str) -> String {
    // Store the map as a vector for later traversal
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for (y, line) in input.lines().enumerate() {
      map.push(line.chars().collect());
      // Store any positions of zeros (trailheads)
      for (x, chr) in line.chars().enumerate() {
        if chr == '0' {
          trailheads.push((x, y));
        }
      }
    }

    // Iterate over the trailheads and score them
    let mut score = 0;
    for trailhead in trailheads {
      // Score the trailhead
      score += Day10::score_subtree(trailhead, &map, &mut Vec::new());
    }

    score.to_string()
  }

  fn solve_part_2(&self, input: &str) -> String {
    // Store the map as a vector for later traversal
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for (y, line) in input.lines().enumerate() {
      map.push(line.chars().collect());
      // Store any positions of zeros (trailheads)
      for (x, chr) in line.chars().enumerate() {
        if chr == '0' {
          trailheads.push((x, y));
        }
      }
    }

    // Iterate over the trailheads and score them
    let mut score = 0;
    for trailhead in trailheads {
      // Score the trailhead
      score += Day10::score_subtree_part_2(trailhead, &map, &mut Vec::new());
    }

    score.to_string()
  }
}

impl Day10 {
  fn score_subtree(trailhead: (usize, usize), map: &Vec<Vec<char>>, visited: &mut Vec<(usize, usize)>) -> i32 {
    visited.push(trailhead);
    let this_space = Day10::get_space((trailhead.0 as i32, trailhead.1 as i32), map).unwrap();
    if this_space == 9 {
      // Base case
      return 1
    } else {
      // Return the sum of all the neighbouring spaces
      let mut score = 0;
      for neighbour in Day10::get_neighbours((trailhead.0 as i32, trailhead.1 as i32)) {
        // Check if neighbour is visitable
        if neighbour.0 >= 0 && neighbour.1 >= 0 {
          if !visited.contains(&(neighbour.0 as usize, neighbour.1 as usize)) {
            let neighbour_space = Day10::get_space(neighbour, map);
            if neighbour_space.is_some() {
              if neighbour_space.unwrap() == this_space + 1 {
                score += Day10::score_subtree((neighbour.0 as usize, neighbour.1 as usize), map, visited);
              }
            }
          }
        }
      }
      score
    }
  }

  fn score_subtree_part_2(trailhead: (usize, usize), map: &Vec<Vec<char>>, visited: &mut Vec<(usize, usize)>) -> i32 {
    let this_space = Day10::get_space((trailhead.0 as i32, trailhead.1 as i32), map).unwrap();
    if this_space == 9 {
      // Base case
      return 1
    } else {
      // Return the sum of all the neighbouring spaces
      let mut score = 0;
      for neighbour in Day10::get_neighbours((trailhead.0 as i32, trailhead.1 as i32)) {
        // Check if neighbour is visitable
        if neighbour.0 >= 0 && neighbour.1 >= 0 {
          if !visited.contains(&(neighbour.0 as usize, neighbour.1 as usize)) {
            let neighbour_space = Day10::get_space(neighbour, map);
            if neighbour_space.is_some() {
              if neighbour_space.unwrap() == this_space + 1 {
                // Create a new visited list for the subtree to use
                let mut new_visited = visited.clone();
                new_visited.push(trailhead);
                score += Day10::score_subtree_part_2((neighbour.0 as usize, neighbour.1 as usize), map, &mut new_visited);
              }
            }
          }
        }
      }
      score
    }
  }

  fn get_space(pos: (i32, i32), map: &Vec<Vec<char>>) -> Option<u32> {
    if pos.0 < 0 || pos.1 < 0 {
      return None
    }
    match map.get(pos.1 as usize) {
      Some(line) => match line.get(pos.0 as usize) {
        Some(space) => space.to_digit(10),
        None => None,
      },
      None => None,
    }
  }

  fn get_neighbours(pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbours: Vec<(i32, i32)> = Vec::new();
    for i in [-1, 1] {
      neighbours.push((pos.0 + i, pos.1));
      neighbours.push((pos.0, pos.1 + i));
    }
    neighbours
  }
}