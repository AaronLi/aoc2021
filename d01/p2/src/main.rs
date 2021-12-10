use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut input_file = File::open("input").expect("File not found");
    let file_lines = io::BufReader::new(&mut input_file).lines();

    let mut num_increases = 0;
    let mut readings = Vec::new();
    for line in file_lines {
        let line = line.unwrap();
        let reading = line.parse::<i32>().unwrap();
        readings.push(reading);
    }

    for i in 1..readings.len()-2 {
        let window_0 = readings[i-1] + readings[i] + readings[i+1];
        let window_1 = readings[i] + readings[i+1] + readings[i+2];
        if window_0 < window_1 {
            num_increases += 1;
        }
    }
    println!("{}", num_increases);
}
