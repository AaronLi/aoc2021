use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

const IFF_String: &str = "Invalid file format";

fn main() {
    let input_file = File::open("input").expect("File not found");
    let mut input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let mut points = HashSet::new();
    for line in &mut input_lines {
        if line == "" {
            break;
        }

        let mut coords = line.split(",");
        let x = coords.next().expect(IFF_String).parse::<isize>().expect(IFF_String);
        let y = coords.next().expect(IFF_String).parse::<isize>().expect(IFF_String);

        let point = (x, y);
        points.insert(point);
    }

    println!("Page points {:?}", points);

    for instruction in input_lines {
        let mut action = instruction.split(" ").last().expect(IFF_String).split("=");

        let axis = action.next().expect(IFF_String);
        let value = action.next().expect(IFF_String).parse::<isize>().expect(IFF_String);
        println!("folding along {}={}", axis, value);

        match axis {
            "y" => {
                let (to_fold, remainder): (HashSet<(isize, isize)>, HashSet<(isize, isize)>) = points.iter().partition(|(_x, y)| y > &value);
                points = remainder;

                for (point_x, point_y) in to_fold {
                    let new_point_y = value + value - point_y;
                    points.insert((point_x, new_point_y));
                }
            },
            "x" => {
                let (to_fold, remainder): (HashSet<(isize, isize)>, HashSet<(isize, isize)>) = points.iter().partition(|(x, _y)| x > &value);
                points = remainder;

                for (point_x, point_y) in to_fold {
                    let new_point_x = value + value - point_x;
                    points.insert((new_point_x, point_y));
                }
            },
            _ => {
                panic!("{}: received axis {}", IFF_String, axis)
            }
        };
        break;
    }   
    println!("{}, {:?}", points.len(), points);
}
