use aoc_lib::aoc_client;
use dotenv::dotenv;

fn main() {
    dotenv().expect("Failed to load .env");

    let input = aoc_client::from_env()
        .expect("Client initialisation failed")
        .year(2024)
        .day(1)
        .get_input()
        .expect("Failed to get puzzle input");
    println!(
        "{}",
        solve_day_1(&input)
    );
}

fn solve_day_1(input: &str) -> i32 {
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

    let mut total = 0;
    // loop {
    //     let smallest_left: usize;
    //     match left_list.iter().enumerate().min_by(|x, y| x.1.cmp(y.1)) {
    //         Some(x) => smallest_left = x.0,
    //         None => break,
    //     }
    //     let smallest_right: usize;
    //     match right_list.iter().enumerate().min_by(|x, y| x.1.cmp(y.1)) {
    //         Some(x) => smallest_right = x.0,
    //         None => break,
    //     }
    //     total +=
    //         (left_list.get(smallest_left).unwrap() - right_list.get(smallest_right).unwrap()).abs();
    //     left_list.remove(smallest_left);
    //     right_list.remove(smallest_right);
    // }
    for number in left_list {
        total += number * right_list.iter().filter(|&x| *x == number).count() as i32;
    }
    total
}
