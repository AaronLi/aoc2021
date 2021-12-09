use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input = File::open("input").expect("File not found");
    let file_lines = io::BufReader::new(input).lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let num_lines: u64 = file_lines.len() as u64;
    let word_length = file_lines[0].len();
    let mut counts = vec![0u64; word_length];

    for line in file_lines {
        for (i, bit) in line.chars().enumerate() {
            print!("{}", bit);
            match bit {
                '1' => counts[i] += 1u64,
                '0' => {},
                _ => panic!("Undefined bit value")
            }
        }
        println!("");
    }
    let mut gamma: u64 = 0;
    for count in counts.iter() {
        gamma = gamma << 1u64;
        if 2 * count > num_lines { // more than half of the numbers in the column are 1
            gamma += 1u64;
        }
    }
    let epsilon: u64 = !gamma & (2u64.pow(word_length as u32) - 1u64);
    println!("Multiplying {} and {}", gamma, epsilon);
    let product: u64 = gamma * epsilon;

    println!("{}", product);
}
