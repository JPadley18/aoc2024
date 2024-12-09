mod days;

use aoc_lib::aoc_client;
use days::{
    day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20, day21,
    day22, day23, day24, day25, day3, day4, day5, day6, day7, day8, day9, PuzzleSolver,
};
use dotenv::dotenv;
use std::{env, time::Instant};

fn main() {
    dotenv().expect("Failed to load .env");
    let mut args = env::args();
    // Skip the program name
    args.next();

    let day: i32 = match args.next() {
        Some(x) => x.parse().expect("Day argument is not a valid integer"),
        None => {
            println!("No day specified, defaulting to Day 1");
            1
        }
    };
    let part: i32 = match args.next() {
        Some(x) => x.parse().expect("Part argument is not a valid integer"),
        None => {
            println!("No part specified, defaulting to part 1");
            1
        }
    };

    // Fetch the input for this day's puzzle
    let input = aoc_client::from_env()
        .expect("Client initialisation failed")
        .year(2024)
        .day(day)
        .get_input()
        .expect("Failed to get puzzle input");

    // Find a solver for this day
    let solver = map_solver(day).unwrap();
    // Time the solution
    let start = Instant::now();
    let solution: Option<String> = match part {
        1 => Some(solver.solve_part_1(&input)),
        2 => Some(solver.solve_part_2(&input)),
        _ => None,
    };
    println!("Day {} Part {}: {}", day, part, solution.unwrap());
    println!("Took {:.2?}", start.elapsed());
}

fn map_solver(day: i32) -> Option<Box<dyn PuzzleSolver>> {
    match day {
        1 => Some(Box::new(day1::Day1)),
        2 => Some(Box::new(day2::Day2)),
        3 => Some(Box::new(day3::Day3)),
        4 => Some(Box::new(day4::Day4)),
        5 => Some(Box::new(day5::Day5)),
        6 => Some(Box::new(day6::Day6)),
        7 => Some(Box::new(day7::Day7)),
        8 => Some(Box::new(day8::Day8)),
        9 => Some(Box::new(day9::Day9)),
        10 => Some(Box::new(day10::Day10)),
        11 => Some(Box::new(day11::Day11)),
        12 => Some(Box::new(day12::Day12)),
        13 => Some(Box::new(day13::Day13)),
        14 => Some(Box::new(day14::Day14)),
        15 => Some(Box::new(day15::Day15)),
        16 => Some(Box::new(day16::Day16)),
        17 => Some(Box::new(day17::Day17)),
        18 => Some(Box::new(day18::Day18)),
        19 => Some(Box::new(day19::Day19)),
        20 => Some(Box::new(day20::Day20)),
        21 => Some(Box::new(day21::Day21)),
        22 => Some(Box::new(day22::Day22)),
        23 => Some(Box::new(day23::Day23)),
        24 => Some(Box::new(day24::Day24)),
        25 => Some(Box::new(day25::Day25)),
        _ => None,
    }
}
