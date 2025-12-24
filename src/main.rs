mod day1;
mod day1_silver;
mod day2;
mod day2_silver;
mod day3;
mod day3_silver;

use std::time;
fn main() {
    println!("Hello, world!");

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