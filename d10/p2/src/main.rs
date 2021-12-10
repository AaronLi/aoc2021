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

    score_map.insert('(', 1);
    score_map.insert('[', 2);
    score_map.insert('{', 3);
    score_map.insert('<', 4);

    let mut scores = Vec::new();
    for line in input_lines{
        let mut stack = Vec::new();
        let mut score: u64 = 0;
        let mut corrupt = false;
        for parentheses in line.chars(){
            match parentheses {
                '(' | '[' | '{' | '<' => stack.push(parentheses),
                '>' | '}' | ']' | ')' => {
                    match stack.last() {
                        Some(top) => {
                            if *top == parentheses_pairs[&parentheses] {
                                stack.pop();
                            }else {
                                corrupt = true;
                                break;
                            }
                        },
                        None => panic!("Too many close brackets!")
                    }
                },
                _ => panic!("Invalid char {}", parentheses)
            }
        }

        if !corrupt {
            for c in stack.iter().rev() {
                score *= 5;
                score += score_map[c];
            }
            scores.push(score);
        }
    }

    scores.sort();

    println!("{}", scores[scores.len()/2]);
}
