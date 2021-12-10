use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let mut parentheses_pairs = HashMap::new();

    parentheses_pairs.insert(')', '(');
    parentheses_pairs.insert(']', '[');
    parentheses_pairs.insert('}', '{');
    parentheses_pairs.insert('>', '<');

    let mut score_map = HashMap::new();

    score_map.insert(')', 3);
    score_map.insert(']', 57);
    score_map.insert('}', 1197);
    score_map.insert('>', 25137);
    let mut score = 0;
    for line in input_lines{
        let mut stack = Vec::new();
        for parentheses in line.chars(){
            match parentheses {
                '(' | '[' | '{' | '<' => stack.push(parentheses),
                '>' | '}' | ']' | ')' => {
                    match stack.last() {
                        Some(top) => {
                            if *top == parentheses_pairs[&parentheses] {
                                stack.pop();
                            }else {
                                score += score_map[&parentheses];
                                break;
                            }
                        },
                        None => panic!("Too many close brackets!")
                    }
                },
                _ => panic!("Invalid char {}", parentheses)
            }
        }
    }

    println!("{}", score);
}
