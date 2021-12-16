use std::fs::File;
use std::io::{self, BufRead};

use std::collections::{BinaryHeap, HashSet};
use std::cmp::{PartialOrd, PartialEq, Ordering, Eq};

#[derive(Eq, Ord, Debug)]
struct AStarPath {
    distance: usize,
    heuristic: usize,
    point: (isize, isize)
}

impl PartialEq for AStarPath {
    fn eq(&self, rhs: &AStarPath) -> bool {
        self.total_weight() == rhs.total_weight()
    }
}

impl PartialOrd for AStarPath {
    fn partial_cmp(&self, rhs: &AStarPath) -> std::option::Option<std::cmp::Ordering> {
        if self.total_weight() < rhs.total_weight() {
            return Some(Ordering::Less);
        }else if self.total_weight() > rhs.total_weight() {
            return Some(Ordering::Greater);
        }else{
            return Some(Ordering::Equal);
        }
    }
}

impl AStarPath {
    fn new(distance: usize, heuristic: usize, point: (isize, isize)) -> AStarPath{
        AStarPath {
            distance,
            heuristic,
            point
        }
    }

    fn total_weight(&self) -> isize {
        -((self.distance + self.heuristic) as isize)
    }
}

const ADJACENT: [(isize, isize);4] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1)
];

fn heuristic(point: (isize, isize), target: (isize, isize)) -> usize{
    let dx = (point.0 - target.0).abs();
    let dy = (point.1 - target.1).abs();
    (dx+dy) as usize
}   

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let mut map = Vec::new();

    for line in input_lines {
        let map_row = line.chars().map(|x| x.to_digit(10).expect("Invalid file format")).collect::<Vec<u32>>();
        map.push(map_row);
    }

    let map_width = map[0].len() as isize;
    let map_height = map.len() as isize;

    let mut frontier = BinaryHeap::new();
    let mut visited = HashSet::new();

    frontier.push(AStarPath::new(0, 0, (0, 0)));

    let mut cost = 0;

    'search: while !frontier.is_empty() {
        let next_hop = frontier.pop().expect("Empty frontier");
        visited.insert(next_hop.point);
        println!("{:?}", next_hop);

        for offset in ADJACENT {
            let adjacent_point = (next_hop.point.0 + offset.0, next_hop.point.1 + offset.1);

            if visited.contains(&adjacent_point){
                continue;
            }
            if !(0..map_width).contains(&adjacent_point.0) {
                continue;
            }

            if !(0..map_height).contains(&adjacent_point.1) {
                continue;
            }

            let location_weight = map[adjacent_point.1 as usize][adjacent_point.0 as usize] as usize;

            let distance_to_adjacent = next_hop.distance + location_weight;

            let heuristic_estimate = heuristic(adjacent_point, (map_width-1, map_height-1));
            if heuristic_estimate == 0{
                cost = distance_to_adjacent;
                break 'search
            }
            let new_node = AStarPath::new(distance_to_adjacent, heuristic_estimate, adjacent_point);
            println!("Adding to frontier: {:?}", new_node);
            frontier.push(new_node);
        }
    }

    println!("{}", cost);
}
