use std::fs;
use std::collections::HashMap;
fn main() {
    let input_line: Vec<isize> = fs::read_to_string("input").expect("File not found").strip_suffix("\n").unwrap().split(",").map(|x| x.parse().expect("Conversion failed")).collect();

    let mut crabs = HashMap::new();

    for sub in input_line {
        let current_count = *crabs.get(&sub).unwrap_or(&0);

        crabs.insert(sub, current_count + 1);

    }

    let mut best_cost = isize::MAX;

    for position in crabs.keys() {
        let mut cost = 0;

        for (source, count) in crabs.iter() {
            cost += (position - source).abs() * count;
        }

        if cost < best_cost {
            best_cost = cost;
        }
    }

    println!("{}", best_cost);
}
