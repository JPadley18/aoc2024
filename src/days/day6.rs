use crate::days::PuzzleSolver;

pub struct Day6;

#[derive(Debug,PartialEq)]
enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT
}

impl Direction {
  fn apply(&self, pos: (i32, i32)) -> (i32, i32) {
    match self {
      Direction::UP => (pos.0, pos.1 - 1),
      Direction::DOWN => (pos.0, pos.1 + 1),
      Direction::LEFT => (pos.0 - 1, pos.1),
      Direction::RIGHT => (pos.0 + 1, pos.1),
    }
  }

  fn rotate(&self) -> Self {
    match self {
      Direction::UP => Direction::RIGHT,
      Direction::DOWN => Direction::LEFT,
      Direction::LEFT => Direction::UP,
      Direction::RIGHT => Direction::DOWN,
    }
  }
}

impl Clone for Direction {
  fn clone(&self) -> Self {
      match self {
        Direction::UP => Direction::UP,
        Direction::DOWN => Direction::DOWN,
        Direction::LEFT => Direction::LEFT,
        Direction::RIGHT => Direction::RIGHT,
      }
  }
}

impl Copy for Direction {}

impl PuzzleSolver for Day6 {
  fn solve_part_1(&self, input: &str) -> String {
      let (board, mut pos) = Day6::parse_input(input);
      let mut direction = Direction::UP;
      println!("{}x{}, player at ({}, {})", board.len(), board.get(0).unwrap().len(), pos.0, pos.1);
      let mut player_outside_board = false;
      let mut visited: Vec<(usize, usize)> = Vec::new();
      while !player_outside_board {
        (pos, direction) = match Day6::move_player(pos, &direction, &board) {
          Some((new_pos, new_direction)) => (new_pos, new_direction),
          None => {
            player_outside_board = true;
            continue;
          }
        };
        if !visited.contains(&pos) {
          visited.push(pos);
        }
      }
      visited.len().to_string()
  }

  fn solve_part_2(&self, input: &str) -> String {
    let (board, mut pos) = Day6::parse_input(r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...");
    let mut direction = Direction::UP;
    println!("{}x{}, player at ({}, {})", board.len(), board.get(0).unwrap().len(), pos.0, pos.1);
    let mut player_outside_board = false;
    let mut visited: Vec<((usize, usize), Direction)> = Vec::new();
    let mut obstacle_positions: Vec<(usize, usize)> = Vec::new();
    while !player_outside_board {
      (pos, direction) = match Day6::move_player(pos, &direction, &board) {
        Some((new_pos, new_direction)) => {
          // First check if there is a loop here
          if !obstacle_positions.contains(&new_pos) && board[new_pos.1][new_pos.0] {
            let mut board_with_obstacle = board.clone();
            board_with_obstacle[new_pos.1][new_pos.0] = false;
            if Day6::path_has_loop(pos, &direction, &board_with_obstacle, &visited) {
              obstacle_positions.push(new_pos);
            }
          }
          (new_pos, new_direction)
        },
        None => {
          player_outside_board = true;
          continue;
        }
      };
      if !visited.contains(&(pos, direction)) {
        visited.push((pos, direction));
      }
    }
    obstacle_positions.len().to_string()
}
}

impl Day6 {
  // Each space in the parsed board is true if it is not an obstacle space
  // OOB is detected when the vec get() fails because Rust cool
  fn parse_input(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut board: Vec<Vec<bool>> = Vec::new();
    let mut player_pos = (0, 0);
    let mut found_player = false;

    for (y, line) in input.lines().enumerate() {
      let mut line_parsed: Vec<bool> = Vec::new();
      for (x, chr) in line.chars().enumerate() {
        match chr {
          '.' => line_parsed.push(true),
          '#' => line_parsed.push(false),
          '^' => {
            found_player = true;
            player_pos = (x, y);
            line_parsed.push(true);
          }
          _ => panic!("Unrecognised character in input"),
        }
      }
      board.push(line_parsed);
    }
    if !found_player {
      panic!("Didn't find player in input");
    }
    (board, player_pos)
  }

  fn move_player(player_pos: (usize, usize), direction: &Direction, board: &Vec<Vec<bool>>) -> Option<((usize, usize), Direction)> {
    let next_pos = direction.apply((player_pos.0 as i32, player_pos.1 as i32));
    if next_pos.0 < 0 || next_pos.1 < 0 {
      return None
    }
    let next_line = match board.get(next_pos.1 as usize) {
      Some(x) => x,
      None => return None,
    };
    let next_space = match next_line.get(next_pos.0 as usize) {
      Some(x) => x,
      None => return None,
    };
    if *next_space {
      return Some(((next_pos.0 as usize, next_pos.1 as usize), *direction))
    }
    Some((player_pos, direction.rotate()))
  }

  fn path_has_loop(pos: (usize, usize), starting_direction: &Direction, board: &Vec<Vec<bool>>, visited: &Vec<((usize, usize), Direction)>) -> bool {
    let mut player_outside_board = false;
    let mut player_pos = pos;
    let mut history = visited.clone();
    let mut direction = *starting_direction;
    while !player_outside_board {
      match Day6::move_player(player_pos, &direction, board) {
        Some((new_pos, new_direction)) => (player_pos, direction) = (new_pos, new_direction),
        None => {
          player_outside_board = true;
          continue;
        }
      };
      if !history.contains(&(player_pos, direction)) {
        history.push((player_pos, direction));
      } else {
        return true
      }
    }
    false
  }
}