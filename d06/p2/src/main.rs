use std::fs;
use std::collections::HashMap;

fn main() {
    let mut fish_state = [0usize; 9];
    let input_fish: Vec<usize> = fs::read_to_string("input").expect("File not found / invalid file format").strip_suffix("\n").unwrap().split(",").map(|x: &str| x.parse().expect("Invalid format")).collect();

    for fish in input_fish {
        fish_state[fish] += 1;
    }


    for day in 0..256 {
        let mut next_state = [0; 9];
        println!("{:?}", fish_state);

        for (timer, count) in fish_state.iter().enumerate() {
            if timer == 0 {
                next_state[6] += count;
                next_state[8] += count;
            }else {
                next_state[timer-1] += count;
            }
        }
        fish_state = next_state;
    }

    let mut total_count = 0;

    for count in fish_state {
        total_count += count;
    }

    println!("{}", total_count);
}
