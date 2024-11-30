use aoc_lib::aoc_client;
use dotenv::dotenv;

fn main() {
    dotenv().expect("Failed to load .env");

    let result = aoc_client::from_env()
        .expect("Client initialisation failed")
        .year(2024)
        .day(1)
        .get_input()
        .expect("Failed to get puzzle input");
    println!("{}", result);
}
