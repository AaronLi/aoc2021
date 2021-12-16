use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let input_file = File::open("input").expect("File not found");
    let mut input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let mut polymer = input_lines.next().expect("Invalid file format").chars().collect::<Vec<char>>();

    let mut insertion_rules = HashMap::new();

    for line in input_lines {
        if line != "" {
            let mut components = line.split(" -> ");

            let from = components.next().expect("Invalid file format").chars().collect::<Vec<char>>();
            let to = components.next().expect("Invalid file format").chars().next().expect("Invalid file format"); // to is only a single char

            insertion_rules.insert(from, to);
        }
    }

    for _step in 0..10 {
        let mut next_polymer = Vec::new();
        for polymer_index in 0..polymer.len()-1 {
            let check_window = &polymer[polymer_index..polymer_index+2];
            let insertion = insertion_rules.get(check_window);
            next_polymer.push(polymer[polymer_index]);
            match insertion {
                Some(new_char) => next_polymer.push(*new_char),
                None => ()
            }
        }
        next_polymer.push(*polymer.last().unwrap());
        polymer = next_polymer;
    }

    let mut element_counts = HashMap::new();

    for c in polymer{
        let count = element_counts.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", element_counts);
    let most_common = element_counts.iter().reduce(|x, y| if x.1 > y.1 {x} else {y}).expect("No most common");
    let least_common = element_counts.iter().reduce(|x, y| if x.1 < y.1 {x} else {y}).expect("No least common");

    println!("{:?} - {:?} = {}", most_common, least_common, most_common.1 - least_common.1);
}
