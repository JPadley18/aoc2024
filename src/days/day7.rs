use crate::days::PuzzleSolver;

pub struct Day7;

#[derive(Debug,Clone,Copy)]
enum Operator {
  ADD,
  MULTIPLY,
  CONCAT,
}

impl Operator {
  fn apply(&self, a: i64, b: i64) -> i64 {
    match self {
      Operator::ADD => a + b,
      Operator::MULTIPLY => a * b,
      Operator::CONCAT => (format!("{}{}", a.to_string(), b.to_string())).parse().unwrap(),
    }
  } 
}

impl PuzzleSolver for Day7 {
  fn solve_part_1(&self, input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
      // Extract the test value and operands
      let mut line_parts = line.split(": ");
      let test_val = line_parts.next().expect("Couldn't read test val").parse().expect("Couldn't parse test value");
      let operands_str = line_parts.next().expect("Couldn't read operand list");
      let operands: Vec<i64> = operands_str.split_whitespace().map(|x| x.parse::<i64>().expect("Couldn't parse operand")).collect();
      if Day7::is_possible(test_val, operands, &vec![Operator::ADD, Operator::MULTIPLY]) {
        total += test_val;
      }
    }
    total.to_string()
  }

  fn solve_part_2(&self, input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
      // Extract the test value and operands
      let mut line_parts = line.split(": ");
      let test_val = line_parts.next().expect("Couldn't read test val").parse().expect("Couldn't parse test value");
      let operands_str = line_parts.next().expect("Couldn't read operand list");
      let operands: Vec<i64> = operands_str.split_whitespace().map(|x| x.parse::<i64>().expect("Couldn't parse operand")).collect();
      if Day7::is_possible(test_val, operands, &vec![Operator::ADD, Operator::MULTIPLY, Operator::CONCAT]) {
        total += test_val;
      }
    }
    total.to_string()
  }
}

impl Day7 {
  fn is_possible(test_val: i64, operands: Vec<i64>, operators: &Vec<Operator>) -> bool {
    for operator_set in Day7::operator_combinations(operators, operands.len() - 1) {
      if Day7::is_equal(test_val, &operands, operator_set) {
        return true
      }
    }
    false
  }

  fn operator_combinations(operators: &Vec<Operator>, len: usize) -> Vec<Vec<Operator>> {
    let mut result: Vec<Vec<Operator>> = Vec::new();
    if len == 1 {
      // Base case
      for operator in operators {
        result.push(vec![*operator]);
      }
    } else {
      let next_results = Day7::operator_combinations(operators, len - 1);

      for operator in operators {
        let new_result_base = vec![*operator];
        for next_result in next_results.iter() {
          let mut new_result = new_result_base.clone();
          new_result.extend(next_result);
          result.push(new_result);
        }
      }
    }
    result
  }

  fn is_equal(test_val: i64, operands: &Vec<i64>, operators: Vec<Operator>) -> bool {
    let mut operand_iterator = operands.iter();
    let mut operator_iterator = operators.iter();
    let mut value = *operand_iterator.next().unwrap();
    for operand in operand_iterator {
      value = operator_iterator.next().unwrap().apply(value, *operand);
    }
    value == test_val
  }
}