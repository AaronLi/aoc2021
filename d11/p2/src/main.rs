use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let mut oct_map = Vec::new();

    for line in input_lines {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(c.to_digit(10).expect("Invalid file format"));
        }
        oct_map.push(row);
    }

    let map_width = oct_map[0].len();
    let map_height = oct_map.len();
    let mut nines = Vec::new();
    let mut step = 0;
    let mut all_flashes = HashSet::new();
    'outer: loop {
        nines.clear();
        all_flashes.clear();
        for (y, row) in oct_map.iter_mut().enumerate(){
            for x in 0..row.len(){
                row[x] += 1;
                if row[x] == 10 {
                    nines.push((x as isize, y as isize));
                    all_flashes.insert((x, y));
                }
            }
        }
        while !nines.is_empty() {
            let iter_nines = nines.clone();
            nines.clear();
            for pos in iter_nines {
                for adj_y in (pos.1 - 1)..(pos.1+2) {
                    for adj_x in (pos.0 - 1) .. (pos.0 + 2) {
                        if (0..map_height as isize).contains(&adj_y) {
                            if (0..map_width as isize).contains(&adj_x) {
                                oct_map[adj_y as usize][adj_x as usize] += 1;
                                if oct_map[adj_y as usize][adj_x as usize] == 10 {
                                    nines.push((adj_x, adj_y));
                                    all_flashes.insert((adj_x as usize, adj_y as usize));
                                }
                            }
                        }
                    }
                }
            }
        }

        for (y, row) in oct_map.iter_mut().enumerate(){
            for x in 0..row.len() {
                if row[x] > 9 {
                    row[x] = 0;
                }
                print!("{}", row[x]);
            }
            println!();
        }
        println!();
        step += 1;

        if all_flashes.len() == map_width * map_height {
            println!("{}", step);
            break 'outer;
        }
    }
}
