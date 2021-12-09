use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines: Vec<Vec<isize>> = io::BufReader::new(input_file).lines().map(|x| x.unwrap().chars().map(|x| x.to_digit(10).unwrap() as isize).collect::<Vec<isize>>()).collect();

    let map_width = input_lines[0].len() as isize;
    let map_height = input_lines.len() as isize;

    let mut minimum_sums = 0;

    let neighbours: [(isize, isize); 4] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1)
    ];

    for y in 0..map_height {
        for x in 0..map_width {
            let mut smaller = true;
            let pos_val = input_lines[y as usize][x as usize];
            for (x_neighbour, y_neighbour) in neighbours {
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
                print!("{}", pos_val);
                minimum_sums += pos_val + 1;
            }
            else{
                print!(" ");
            }
        }
        println!();
    }

    println!("{}", minimum_sums)
}
