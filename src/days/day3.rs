use crate::days::PuzzleSolver;
use regex::Regex;

pub struct Day3;

impl PuzzleSolver for Day3 {
    fn solve_part_1(&self, input: &str) -> String {
        let mult_re = Regex::new(r"(mul\((\d+),(\d+)\))").expect("Regex failed to compile for part 1");
        Day3::multiply_matches(mult_re, input).to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        // Not as neat as I'd like :(
        let disable_re = Regex::new(r"(don't)\(\)()()|(do)\(\)()()|(mul)\((\d+),(\d+)\)").expect("Regex failed to compile for part 2");
        let mut disable = false;
        let mut total = 0;
        for (_, [command, a_str, b_str]) in disable_re.captures_iter(input).map(|x| x.extract()) {
            match command {
                "don't" => disable = true,
                "do" => disable = false,
                "mul" => if !disable {
                    total += a_str.parse::<i32>().unwrap() * b_str.parse::<i32>().unwrap();
                },
                // Note to self - there is probably a better way to handle this case than this, but
                // it will never be called so who cares
                _ => panic!("how"),
            }
        }
        total.to_string()
    }
}

impl Day3 {
    fn multiply_matches(re: Regex, input: &str) -> i32 {
        let mut total = 0;
        for (_, [_, a_str, b_str]) in re.captures_iter(input).map(|x| x.extract()) {
            total += a_str.parse::<i32>().unwrap() * b_str.parse::<i32>().unwrap();
        }
        total
    }
}