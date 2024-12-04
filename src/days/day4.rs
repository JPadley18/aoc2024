use crate::days::PuzzleSolver;

pub struct Day4;

impl PuzzleSolver for Day4 {
    fn solve_part_1(&self, input: &str) -> String {
        let as_lines = input.lines().collect::<Vec<&str>>();
        let width = as_lines.get(0).unwrap().len();
        let height = as_lines.len();
        let mut total = 0;
        // Count up all the horizontal matches
        for x in 0..width - 3 {
            for y in 0..height {
                let segment = &as_lines.get(y).unwrap()[x..x + 4];
                if segment == "XMAS" || segment == "SAMX" {
                    total += 1
                }
            }
        }
        // Count up all the vertical matches
        for x in 0..width {
            for y in 0..height - 3 {
                let segment = String::from_iter(
                    as_lines
                        .iter()
                        .skip(y)
                        .take(4)
                        .map(|&line| line.chars().skip(x).next().unwrap())
                        .collect::<Vec<_>>(),
                );
                if segment == "XMAS" || segment == "SAMX" {
                    total += 1
                }
            }
        }
        // Count up all the diagonal matches
        for x in 0..width - 3 {
            for y in 0..height - 3 {
                // Check the backslash-shaped ones
                let mut segment = String::from_iter(
                    as_lines
                    .iter()
                    .skip(y)
                    .take(4)
                    .enumerate()
                    .map(|(i, &line)| line.chars().skip(x + i).next().unwrap()),
                );
                if segment == "XMAS" || segment == "SAMX" {
                    total += 1
                }
                // Check the forward-slash-shaped ones
                segment = String::from_iter(
                    as_lines
                        .iter()
                        .skip(y)
                        .take(4)
                        .enumerate()
                        .map(|(i, &line)| line.chars().skip(x + (3 - i)).next().unwrap()),
                );
                if segment == "XMAS" || segment == "SAMX" {
                    total += 1
                }
            }
        }
        total.to_string()
    }

    fn solve_part_2(&self, input: &str) -> String {
        let as_lines = input.lines().collect::<Vec<&str>>();
        let width = as_lines.get(0).unwrap().len();
        let height = as_lines.len();
        let mut total = 0;
        for x in 0..width - 2 {
            for y in 0..height - 2 {
                let first_segment = String::from_iter(
                    as_lines
                    .iter()
                    .skip(y)
                    .take(3)
                    .enumerate()
                    .map(|(i, &line)| line.chars().skip(x + i).next().unwrap()),
                );
                // Check the forward-slash-shaped ones
                let second_segment = String::from_iter(
                    as_lines
                        .iter()
                        .skip(y)
                        .take(3)
                        .enumerate()
                        .map(|(i, &line)| line.chars().skip(x + (2 - i)).next().unwrap()),
                );
                if (first_segment == "MAS" || first_segment == "SAM") && (second_segment == "MAS" || second_segment == "SAM") {
                    total += 1
                }
            }
        }
        total.to_string()
    }
}
