use crate::days::PuzzleSolver;

pub struct Day1;

impl PuzzleSolver for Day1 {
    fn solve_part_1(&self, input: &str) -> String {
        let (mut left_list, mut right_list) = self.parse_lists(input);
        let mut total = 0;
        loop {
            let smallest_left: usize;
            match left_list.iter().enumerate().min_by(|x, y| x.1.cmp(y.1)) {
                Some(x) => smallest_left = x.0,
                None => break,
            }
            let smallest_right: usize;
            match right_list.iter().enumerate().min_by(|x, y| x.1.cmp(y.1)) {
                Some(x) => smallest_right = x.0,
                None => break,
            }
            total +=
                (left_list.get(smallest_left).unwrap() - right_list.get(smallest_right).unwrap()).abs();
            left_list.remove(smallest_left);
            right_list.remove(smallest_right);
        }
        total.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let (left_list, right_list) = self.parse_lists(input);
        left_list
            .iter()
            .map(|x| x * right_list.iter().filter(|&y| *y == *x).count() as i32)
            .sum::<i32>()
            .to_string()
    }
}

impl Day1 {
    fn parse_lists(&self, input: &str) -> (Vec<i32>, Vec<i32>) {
        let mut left_list: Vec<i32> = Vec::new();
        let mut right_list: Vec<i32> = Vec::new();
        for (i, line) in input.lines().enumerate() {
            let mut parts = line.splitn(2, "   ");
            let left_num: i32 = parts
                .next()
                .expect(&format!("No first number found on line {}", i))
                .parse()
                .expect(&format!("Coulen't parse first number on line {}", i));
            let right_num: i32 = parts
                .next()
                .expect(&format!("No second number found on line {}", i))
                .parse()
                .expect(&format!("Couldn't parse second number on line {}", i));
            left_list.push(left_num);
            right_list.push(right_num);
        }
        (left_list, right_list)
    }
}
