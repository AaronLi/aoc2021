use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input = File::open("input").expect("File not found");
    let file_lines = io::BufReader::new(input).lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let word_length = file_lines[0].len();
    let mut lines = Vec::new();

    for line in file_lines {
        lines.push(line);
    }

    let oxygen_generator = reduce_readings(&lines, word_length, BitCriteria::MostCommon);

    let c02_scrubber = reduce_readings(&lines, word_length, BitCriteria::LeastCommon);

    let product: u32 = oxygen_generator * c02_scrubber;

    println!("{}", product);
}

enum BitCriteria {
    MostCommon,
    LeastCommon
}


fn reduce_readings(readings: &[String], word_length: usize, criteria: BitCriteria) -> u32{
    let mut in_consideration = Vec::new();
    for i in 0..readings.len() {
        in_consideration.push(i);
    }

    for column in 0..word_length {
        let num_1_in_column = (&in_consideration).into_iter().filter(|x| readings[**x].chars().nth(column).expect("incorrect word length") == '1').count();
        let most_common_1 = num_1_in_column * 2 > in_consideration.len();
        let count_equal = num_1_in_column * 2 == in_consideration.len();
        in_consideration = in_consideration.into_iter().filter(|x| {
            let char_is_1 = (readings[*x].chars().nth(column).expect("incorrect word length") == '1') ^ matches!(criteria, BitCriteria::LeastCommon);
            
            if most_common_1 && char_is_1 {
                return true;
            }else if !most_common_1 && !count_equal && !char_is_1 { // most common is 0 and char is 0
                return true;
            }else if char_is_1 && count_equal {
                return true;
            }else{
                return false;
            }
        }).collect();

        if in_consideration.len() == 1 {
            break
        }
    }

    let mut num_out = 0u32;

    for bit in readings[in_consideration[0]].chars() {
        num_out <<= 1;
        num_out += (bit == '1') as u32;
    }

    num_out
}