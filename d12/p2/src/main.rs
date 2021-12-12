use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn traverse_cave(neighbours: &HashMap<String, Vec<String>>, major_caves: &HashSet<String>, start: &String, visited: &mut (HashSet<String>, HashSet<String>)) -> usize {
    if start == "end"{
        return 1;
    }
    let mut ways_from_here = 0;
    for adjacent in neighbours.get(start).expect("Invalid neighbour table format") {
        let mut can_visit = true;
        if !major_caves.contains(adjacent)  {
            if visited.0.contains(adjacent) && !visited.1.is_empty() {
                can_visit = false;
            }else if visited.1.contains(adjacent) {
                can_visit = false;
            }else if adjacent == "start" {
                can_visit = false;
            }
        }

        if can_visit {
            //println!("Going from {} to {}, I've visited {:?}", start, adjacent, visited);
            if visited.0.contains(adjacent) {
                if !major_caves.contains(adjacent){
                    visited.1.insert(adjacent.clone());
                }
                ways_from_here += traverse_cave(neighbours, major_caves, adjacent, visited);
                visited.1.remove(adjacent);
            }else{
                visited.0.insert(adjacent.clone());
                ways_from_here += traverse_cave(neighbours, major_caves, adjacent, visited);
                visited.0.remove(adjacent);
            }
            
        }
    }
    ways_from_here
}

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let mut neighbours:HashMap<String, Vec<String>> = HashMap::new();
    let mut major_caves = HashSet::new();
    for line in input_lines {
        let mut endpoints = line.split("-");

        let start = String::from(endpoints.next().expect("Invalid file format"));
        let end = String::from(endpoints.next().expect("Invalid file format"));

        // looks nicer, but really we only need to check a single char
        if start.chars().all(|c: char| c.is_ascii_uppercase()) {
            major_caves.insert(start.clone());
        }

        if end.chars().all(|c: char| c.is_ascii_uppercase()) {
            major_caves.insert(end.clone());
        }

        match neighbours.get_mut(&start) {
            Some(vec) => vec.push(end.clone()),
            None => {
                let mut new_vec = Vec::new();
                new_vec.push(end.clone());
                neighbours.insert(start.clone(), new_vec);
            }
        };

        match neighbours.get_mut(&end) {
            Some(vec) => vec.push(start),
            None => {
                let mut new_vec = Vec::new();
                new_vec.push(start);
                neighbours.insert(end, new_vec);
            }
        };

    }
    let mut visited_set = (HashSet::new(), HashSet::new());
    visited_set.0.insert(String::from("start"));
    let num_paths = traverse_cave(&neighbours, &major_caves, &String::from("start"), &mut visited_set);

    println!("{}", num_paths);

}