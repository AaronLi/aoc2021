use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let input_file = File::open("input").expect("File not found");
    let mut input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let input_chars = input_lines.next().unwrap().chars().collect::<Vec<char>>();

    let mut polymer = HashMap::new();

    let first_char = input_chars[0];

    for i in 0..input_chars.len()-1 {
        let symbol = (input_chars[i], input_chars[i+1]);
        let count = polymer.entry(symbol).or_insert(0usize);
        *count += 1;
    }

    let mut insertion_rules = HashMap::new();

    for line in input_lines {
        if line != "" {
            let mut components = line.split(" -> ");

            let from = components.next().expect("Invalid file format").chars().collect::<Vec<char>>();
            let to = components.next().expect("Invalid file format").chars().next().expect("Invalid file format"); // to is only a single char

            insertion_rules.insert((from[0], from[1]), to);
        }
    }

    for _step in 0..40 {
        let mut next_polymer = HashMap::new();
        for (pair, count) in polymer {
            let rule = insertion_rules.get(&pair);

            match rule {
                Some(replacement) => {
                    let new_symbol = (pair.0, *replacement);
                    let new_symbol_count = next_polymer.entry(new_symbol).or_insert(0);
                    *new_symbol_count += count;

                    let new_symbol = (*replacement, pair.1);
                    let new_symbol_count = next_polymer.entry(new_symbol).or_insert(0);
                    *new_symbol_count += count;
                },
                None => {
                    next_polymer.insert(pair, count);
                }
            }
        }
        polymer = next_polymer;
        println!("{}", _step);
    }

    let mut element_counts = HashMap::new();

    for (pair, count) in polymer{
        let current_count = element_counts.entry(pair.1).or_insert(0);
        *current_count += count;
    }
    *element_counts.entry(first_char).or_insert(0) += 1;
    
    println!("{:?}", element_counts);
    let most_common = element_counts.iter().reduce(|x, y| if x.1 > y.1 {x} else {y}).expect("No most common");
    let least_common = element_counts.iter().reduce(|x, y| if x.1 < y.1 {x} else {y}).expect("No least common");

    println!("{:?} - {:?} = {}", most_common, least_common, most_common.1 - least_common.1);
}
