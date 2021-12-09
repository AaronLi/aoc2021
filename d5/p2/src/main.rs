use std::io::{self, BufRead};
use std::fs::File;

use std::collections::HashMap;

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let mut grid: HashMap<(isize, isize), usize> = HashMap::new();
    let mut multicross = 0;
    for line in input_lines {
        let line_endpoints: Vec<Vec<&str>> = line.split(" -> ").map(|x| x.split(",").collect()).collect();

        let line_start = &line_endpoints[0];
        let line_start_x: isize = line_start[0].parse().unwrap();
        let line_start_y: isize = line_start[1].parse().unwrap();
        let line_end = &line_endpoints[1];
        let line_end_x: isize = line_end[0].parse().unwrap();
        let line_end_y: isize = line_end[1].parse().unwrap();

        let line_diff_x = line_end_x - line_start_x;
        let line_diff_y = line_end_y - line_start_y;

        let num_steps = line_diff_x.abs().max(line_diff_y.abs());
        let step_x = if line_diff_x != 0 {line_diff_x/line_diff_x.abs()} else {0};
        let step_y = if line_diff_y != 0 {line_diff_y / line_diff_y.abs()} else {0};

        for i in 0..num_steps + 1 {
            let grid_pos = (line_start_x + i * step_x, line_start_y + i * step_y);
            let current_val = *grid.get(&grid_pos).unwrap_or(&0usize);
            if current_val == 1 {
                multicross += 1;
            }
            (&mut grid).insert(grid_pos, current_val + 1);
        }
    }

    println!("{}", multicross);
}
