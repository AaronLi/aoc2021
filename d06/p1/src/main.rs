use std::fs;
use std::collections::HashMap;

fn main() {
    let mut fish_state: HashMap<usize, usize> = HashMap::new();
    let input_fish: Vec<usize> = fs::read_to_string("input").expect("File not found / invalid file format").strip_suffix("\n").unwrap().split(",").map(|x: &str| x.parse().expect("Invalid format")).collect();

    for fish in input_fish {
        let current_count = *fish_state.get(&fish).unwrap_or(&0);

        fish_state.insert(fish, current_count +1);
    }


    for day in 0..80 {
        let mut next_state: HashMap<usize, usize> = HashMap::new();
        println!("{:?}", fish_state);

        for (timer, count) in fish_state.iter() {
            if *timer == 0 {
                let current_value = *next_state.get(&6).unwrap_or(&0);
                next_state.insert(6, current_value + count);
                next_state.insert(8, *count);
            }else {
                let current_value = *next_state.get(&(timer - 1)).unwrap_or(&0);
                next_state.insert(timer-1, count+current_value);
            }
        }
        fish_state = next_state;
    }

    let mut total_count = 0;

    for (timer, count) in fish_state.iter() {
        total_count += count;
    }

    println!("{}", total_count);
}
