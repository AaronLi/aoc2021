use std::fs;
use std::collections::HashMap;
fn main() {
    let input_line: Vec<isize> = fs::read_to_string("input").expect("File not found").trim().split(",").map(|x| x.parse().expect("Conversion failed")).collect();

    let mut crabs = HashMap::new();

    let mut min_pos = isize::MAX;
    let mut max_pos = isize::MIN;

    for sub in input_line {
        let current_count = *crabs.get(&sub).unwrap_or(&0);

        crabs.insert(sub, current_count + 1);

        min_pos = min_pos.min(sub);
        max_pos = max_pos.max(sub);
    }

    let mut best_cost = isize::MAX;

    for position in min_pos..max_pos+1 {
        let mut cost = 0;

        for (source, count) in crabs.iter() {
            let move_amount = (position - source).abs();
            let move_cost = (move_amount * (move_amount + 1))/2 * count;
            cost += move_cost;
            //println!("Move from {} to {}: {} fuel (x{})", source, position, move_cost, count);
        }

        if cost < best_cost {
            best_cost = cost;
        }
    }

    println!("{}", best_cost);
}
