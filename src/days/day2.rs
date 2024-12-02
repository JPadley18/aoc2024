use crate::days::PuzzleSolver;

pub struct Day2;

impl PuzzleSolver for Day2 {
    fn solve_part_1(&self, input: &str) -> String {
        let parsed_input = self.parse_input(input);
        parsed_input.iter().filter(|x| self.input_is_safe(x)).count().to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let parsed_input = self.parse_input(input);
        parsed_input.iter().filter(|x| self.input_is_safe_with_dampener(x)).count().to_string()
    }
}

impl Day2 {
    fn parse_input(&self, input: &str) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for line in input.lines() {
            let mut parsed_line: Vec<i32> = Vec::new();
            for number in line.split_whitespace() {
                parsed_line.push(number.parse().expect("Hit an invalid number in input"));
            }
            result.push(parsed_line);
        }
        result
    }

    fn input_is_safe(&self, input: &Vec<i32>) -> bool {
        self.where_is_input_unsafe(input).is_none()
    }

    fn input_is_safe_with_dampener(&self, input: &Vec<i32>) -> bool {
        match self.where_is_input_unsafe(input) {
            None => return true,
            Some(i) => {
                // Try removing each element from this one back until we find a working version
                for j in (0..(i + 1)).rev() {
                    let mut temp = input.clone();
                    temp.remove(j as usize);
                    if self.input_is_safe(&temp) {
                        return true
                    }
                }
                false
            }
        }
    }

    fn where_is_input_unsafe(&self, input: &Vec<i32>) -> Option<i32> {
        let mut it = input.iter().enumerate();
        let mut last = it.next().expect("Vector is empty").1;
        let mut direction = 0;
        for (i, current) in it {
            let delta = current - last;
            let new_direction = delta.signum();
            let mag = delta.abs();
            if (mag > 3 || mag < 1) || (direction != 0 && new_direction != direction) {
                return Some(i as i32);
            }
            direction = new_direction;
            last = current;
        }
        None
    }
}
