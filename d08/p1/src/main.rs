use std::fs::File;
use std::io::{self, BufRead};
fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines:Vec<String> = io::BufReader::new(input_file).lines().map(|x| x.unwrap()).collect();

    let mut num_digits = 0;

    for line in input_lines{
        let line_components: Vec<&str> = line.split(" | ").collect();

        let _signal_patterns = line_components[0];
        let output_values = line_components[1].split(" ");

        for output in output_values {
            match output.len() {
                2 | 4 | 3 | 7 => {
                    num_digits += 1;
                },
                _ => {}
            }
        }
    }


    println!("{}", num_digits);
}
