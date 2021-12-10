use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

const NEIGHBOURS: [(isize, isize); 4] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1)
    ];

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines: Vec<Vec<isize>> = io::BufReader::new(input_file).lines().map(|x| x.unwrap().chars().map(|x| x.to_digit(10).unwrap() as isize).collect::<Vec<isize>>()).collect();

    let map_width = input_lines[0].len() as isize;
    let map_height = input_lines.len() as isize;

    let mut basin_sizes = Vec::new();

    let mut visited = HashSet::new();

    for y in 0..map_height {
        for x in 0..map_width {
            let mut smaller = true;
            let pos_val = input_lines[y as usize][x as usize];
            for (x_neighbour, y_neighbour) in NEIGHBOURS {
                let check_x = x + x_neighbour;
                let check_y = y + y_neighbour;
                if (0..map_width).contains(&check_x) && (0..map_height).contains(&check_y) {
                    if pos_val >= input_lines[check_y as usize][check_x as usize] {
                        smaller = false;
                        break;
                    }
                }
            }

            if smaller {
                let basin_size = find_basin(&input_lines[..], x, y, &mut visited, map_width, map_height);
                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.sort();

    basin_sizes.reverse();

    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn find_basin(grid: &[Vec<isize>], start_x: isize, start_y: isize, visited: &mut HashSet<(isize, isize)>, map_width: isize, map_height: isize) -> isize{
    let mut basin_size = 1;

    if visited.contains(&(start_x as isize, start_y as isize)){
        return 0;
    }else if grid[start_y as usize][start_x as usize] == 9 {
        return 0;
    }
    visited.insert((start_x, start_y));

    for neighbour in NEIGHBOURS {
        let neighbour_pos_x = start_x + neighbour.0;
        let neighbour_pos_y = start_y + neighbour.1;

        if (0..map_width).contains(&neighbour_pos_x){
            if (0..map_height).contains(&neighbour_pos_y){
                basin_size += find_basin(grid, neighbour_pos_x, neighbour_pos_y, visited, map_width, map_height);
            }
        }
    }

    return basin_size;
}