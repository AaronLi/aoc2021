use std::fs::File;
use std::io::{BufRead, self};

fn main() {
    let file = File::open("input").expect("File not found");
    let file_lines = io::BufReader::new(file).lines();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in file_lines {
        let command_components = line.unwrap();
        let command: Vec<&str> = command_components.split(" ").collect();

        let amount = command[1].parse::<i32>().unwrap();
        match command[0] {
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            },
            "up" => aim -= amount,
            "down" => aim += amount,
            default => panic!("Unknown command: {}", default),
        }

    }
    let output = horizontal * depth;
    println!("{}", output);
}
