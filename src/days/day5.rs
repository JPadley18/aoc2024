use crate::days::PuzzleSolver;

pub struct Day5;

impl PuzzleSolver for Day5 {
    fn solve_part_1(&self, input: &str) -> String {
      let (rules, runs) = Day5::parse_input(input);
      let mut total = 0;
      for run in runs {
        if Day5::run_is_in_order(&run, &rules) {
          let mid_idx = run.len() / 2;
          total += run.get(mid_idx).unwrap();
        }
      }
      total.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
      let (rules, runs) = Day5::parse_input(input);
      let mut total = 0;
      for mut run in runs {
        if !Day5::run_is_in_order(&run, &rules) {
          while !Day5::run_is_in_order(&run, &rules) {
            // Reorder the failed run
            run = Day5::reorder(run, &rules);
          }
          let mid_idx = run.len() / 2;
          total += run.get(mid_idx).unwrap();
        }
      }
      total.to_string()
    }
}

impl Day5 {
    fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let mut rules: Vec<(i32, i32)> = Vec::new();
        let mut runs: Vec<Vec<i32>> = Vec::new();

        let mut parsing_rules = true;
        for line in input.lines() {
            if line.len() == 0 {
                parsing_rules = false;
            } else if parsing_rules {
                let mut split = line.split("|");
                let first = split.next().unwrap().parse::<i32>().unwrap();
                let second = split.next().unwrap().parse::<i32>().unwrap();
                rules.push((first, second));
            } else {
                let mut run: Vec<i32> = Vec::new();
                for num in line.split(",") {
                  run.push(num.parse::<i32>().unwrap());
                }
                runs.push(run);
            }
        }

        (rules, runs)
    }

    fn run_is_in_order(run: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
      for rule in rules {
        let contains_0 = Day5::vec_index_of(run, &rule.0);
        let index_0;
        match contains_0 {
          Some(i) => index_0 = i,
          None => continue,
        };
        let contains_1 = Day5::vec_index_of(run, &rule.1);
        let index_1;
        match contains_1 {
          Some(i) => index_1 = i,
          None => continue,
        };

        if index_0 > index_1 {
          return false;
        }
      }
      true
    }

    fn vec_index_of(vec: &Vec<i32>, value: &i32) -> Option<usize> {
      vec.iter().position(|x| x == value)
    }

    fn reorder(mut run: Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
      for rule in rules {
        let contains_0 = Day5::vec_index_of(&run, &rule.0);
        let index_0;
        match contains_0 {
          Some(i) => index_0 = i,
          None => continue,
        };
        let contains_1 = Day5::vec_index_of(&run, &rule.1);
        let index_1;
        match contains_1 {
          Some(i) => index_1 = i,
          None => continue,
        };

        if index_0 > index_1 {
          // Swap the offending entries
          run[index_0] = rule.1;
          run[index_1] = rule.0;
        }
      }
      run
    }
}
