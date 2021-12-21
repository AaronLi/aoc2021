use std::fmt::Display;
use std::ops::Sub;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
// I sure wish I had numpy right now...

#[derive(Copy, Clone)]
enum Rotation {
    _0 = 0,
    _90 = 90,
    _180 = 180,
    _270 = 270
}

impl Display for Rotation {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
         write!(f, "{}", *self as isize)
    }
}
#[derive(Copy, Clone, Debug, Hash, Eq)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize
}

impl PartialEq for Point3D {
    
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl Sub for Point3D {
    
    type Output = Point3D;
    
    fn sub(self, rhs: Point3D) -> <Self as std::ops::Sub<Point3D>>::Output {
        Point3D{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

#[derive(Copy, Clone)]
struct RotationMatrix {
    matrix: [[isize; 4]; 3]
}

impl RotationMatrix {

    fn new(parts: &[[isize;4];3]) -> Self {
        RotationMatrix{
            matrix: parts.clone()
        }
    }

    fn identity() -> Self {
        RotationMatrix::new(
            &[
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0]
            ]
        )
    }

    fn new_rotate(rotate_x: Rotation, rotate_y: Rotation, rotate_z: Rotation) -> Self {
        let cos_x = (rotate_x as isize as f64).to_radians().cos() as isize; // cos c
        let sin_x = (rotate_x as isize as f64).to_radians().sin() as isize; // sin c

        let cos_y = (rotate_y as isize as f64).to_radians().cos() as isize; // cos b
        let sin_y = (rotate_y as isize as f64).to_radians().sin() as isize; // sin b

        let cos_z = (rotate_z as isize as f64).to_radians().cos() as isize; // cos a
        let sin_z = (rotate_z as isize as f64).to_radians().sin() as isize; // sin a

        RotationMatrix::new(
            &[
                [cos_z*cos_y, cos_z * sin_y  * sin_x - sin_z * cos_x, cos_z*sin_y*cos_x + sin_z*sin_x, 0],
                [sin_z*cos_y, sin_z*sin_y*sin_x + cos_z*cos_x, sin_z*sin_y*cos_x - cos_z*sin_x, 0],
                [-sin_y, cos_y*sin_x, cos_y*cos_x, 0]
            ]
        )
    }

    fn apply(&self, point: &Point3D) -> Point3D{
        Point3D{
            x: self.matrix[0][0] * point.x + self.matrix[0][1] * point.y + self.matrix[0][2] * point.z + self.matrix[0][3],
            y: self.matrix[1][0] * point.x + self.matrix[1][1] * point.y + self.matrix[1][2] * point.z + self.matrix[1][3],
            z: self.matrix[2][0] * point.x + self.matrix[2][1] * point.y + self.matrix[2][2] * point.z + self.matrix[2][3]
        }
    }

    fn translate(&mut self, translate: Point3D) -> &mut RotationMatrix{
        self.matrix[0][3] += translate.x;
        self.matrix[1][3] += translate.y;
        self.matrix[2][3] += translate.z;
        self
    }
}

impl Display for RotationMatrix {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut out_string = String::new();
        for row in self.matrix{
            for value in row {
                out_string += format!("{:4} ", value).as_str();
            }
            out_string += "\n";
        };
        write!(f, "{}", out_string)
    }
}

#[derive(Debug)]
struct Scanner {
    readings: HashSet<Point3D>,
}

impl Scanner {
    fn new() -> Self {
        Scanner{
            readings: HashSet::new(),
        }
    }

    fn num_points(&self) -> usize {
        self.readings.len()
    }

    fn get_rotated_points(&self, rotation: &RotationMatrix) -> HashSet<Point3D>{
        self.readings.iter().map(|x| rotation.apply(x)).collect::<HashSet<Point3D>>()
    }

    fn points_matching(&self, other: &Scanner, rotate_other: &RotationMatrix) -> (usize, RotationMatrix) {
        let mut best_match: Option<(usize, RotationMatrix)> = None;
        for target in &self.readings {
            for source in &other.readings {
                // assume y and x are the same point, move them together
                let mut transform_temp = rotate_other.clone();
                let translation = *target - transform_temp.apply(source);
                transform_temp.translate(translation);
                let check_points = other.get_rotated_points(&transform_temp);

                let matches = self.readings.intersection(&check_points).count();
                if best_match.is_none() || matches > best_match.unwrap().0 {
                    best_match = Some((matches, transform_temp));
                }
            }
        }
        best_match.unwrap()
    }

    fn add_points(&mut self, transform: &RotationMatrix, other: &Scanner){
        self.readings.extend(other.get_rotated_points(transform));
    }
}

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());
    
    let mut scanners = Vec::new();

    for line in input_lines {
        if line.starts_with("---") {
            scanners.push(Scanner::new());
        }else{
            let line_parts:Vec<&str> = line.split(",").collect();
            if line_parts.len() != 3 {
                continue
            }

            scanners.last_mut().unwrap().readings.insert(Point3D{
                x: line_parts[0].parse().expect("Invalid file format"),
                y: line_parts[1].parse().expect("Invalid file format"),
                z: line_parts[2].parse().expect("Invalid file format")
            });
        }
    }
    let mut base_scanner = scanners.pop().expect("No scanners");
    while scanners.len() > 0{
        for x_rotation in [Rotation::_0, Rotation::_90, Rotation::_180, Rotation::_270] {
            for y_rotation in [Rotation::_0, Rotation::_90, Rotation::_180, Rotation::_270] {
                for z_rotation in [Rotation::_0, Rotation::_90, Rotation::_180, Rotation::_270] {
                    let temp_rotation = RotationMatrix::new_rotate(x_rotation, y_rotation, z_rotation);
                    for scanner_index in (0..scanners.len()).rev() {
                        let points_matching = base_scanner.points_matching(&scanners[scanner_index], &temp_rotation);
                        if points_matching.0 > 11 {
                            base_scanner.add_points(&points_matching.1, &scanners.remove(scanner_index));
                            println!("{} scanners left", scanners.len());
                        }
                    }
                }
            }
        }
    }
    println!("{}", base_scanner.readings.len());
}
