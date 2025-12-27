mod day1;
mod day1_silver;
mod day2;
mod day2_silver;
mod day3;
mod day3_silver;
mod day4;
mod day4_silver;
mod day5;
mod day5_silver;
mod day6;
mod day6_silver;
mod day7;
mod day7_silver;
mod day8;
mod day8_silver;
mod day9;
mod day9_silver;
mod day10;
mod day10_silver;

use std::time;
fn main() {
    println!("Hello, world!");
    let total_time_elapsed = time::Instant::now();

    //DAY 1
    let start = time::Instant::now();
    dbg!(day1::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day1_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);

    // DAY 2
    let start = time::Instant::now();
    dbg!(day2::solution());
    let duration = start.elapsed();
    dbg!(duration);
    // let start = time::Instant::now();
    // dbg!(day2_silver::solution());
    // let duration = start.elapsed();
    // dbg!(duration);

    // DAY 3
    let start = time::Instant::now();
    dbg!(day3::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day3_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);

    // DAY 4
    let start = time::Instant::now();
    dbg!(day4::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day4_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);

    // DAY 5
    let start = time::Instant::now();
    dbg!(day5::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day5_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);

    // DAY 6
    let start = time::Instant::now();
    dbg!(day6::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day6_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);

    // DAY 7
    let start = time::Instant::now();
    dbg!(day7::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day7_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);

    // DAY 8
    let start = time::Instant::now();
    dbg!(day8::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day8_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);

    // DAY 9
    let start = time::Instant::now();
    dbg!(day9::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day9_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);

    // DAY 10
    let start = time::Instant::now();
    dbg!(day10::solution());
    let duration = start.elapsed();
    dbg!(duration);
    let start = time::Instant::now();
    dbg!(day10_silver::solution());
    let duration = start.elapsed();
    dbg!(duration);


    dbg!(total_time_elapsed.elapsed());
}


use std::fs;
pub fn load_input(n: i32) -> Vec<u8>{
    let x = fs::read(format!("./inputs/{}.txt", n.to_string()));
    return x.unwrap();
}
pub fn lines_from_bytes(mut data: Vec<u8>) -> Vec<Vec<u8>> {
    let mut lines = Vec::new();

    while let Some(pos) = data[0..].iter().position(|&b| b == b'\n') {
        let end = pos;
        lines.push(data.drain(0..=end).collect());
        // No need for start = end + 1; drain adjusts remaining data
    }

    // Last line
    if !data.is_empty() {
        lines.push(data.drain(..).collect());
    }

    lines
}
pub fn lines_from_comma_separated_bytes(mut data: Vec<u8>) -> Vec<Vec<u8>> {
    let mut lines = Vec::new();

    while let Some(pos) = data[0..].iter().position(|&b| b == b',') {
        let end = pos;
        lines.push(data.drain(0..=end).collect());
        // No need for start = end + 1; drain adjusts remaining data
    }

    // Last line
    if !data.is_empty() {
        lines.push(data.drain(..).collect());
    }

    lines
}