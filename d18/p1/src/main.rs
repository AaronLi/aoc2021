use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use serde_json::{Value};

#[derive(Debug)]
enum NumberOrNest{
    Number(u64),
    Nest(Vec<NumberOrNest>)
}

enum ExplodeSide{
    Left,
    Right
}

impl NumberOrNest {

    fn parse(in_json: Value) -> Self {
        match in_json {
            Value::Number(v) => NumberOrNest::Number(v.as_u64().expect("Number too large")),
            Value::Array(a) => {
                let mut vec_out = Vec::new();
                for child in a {
                    vec_out.push(NumberOrNest::parse(child));
                }

                NumberOrNest::Nest(vec_out)
            },
            _ => panic!("Invalid snailfish element")
        }
    }

    fn _explode_dive(&mut self, side: ExplodeSide, value: u64) {
        match side {
            ExplodeSide::Left => {
                match self {
                    NumberOrNest::Number(v) => {
                        *v += value;
                    },
                    NumberOrNest::Nest(v) => {
                        v[1]._explode_dive(side, value);
                    }
                }
            },
            ExplodeSide::Right => {
                match self {
                    NumberOrNest::Number(v) => {
                        *v += value;
                    },
                    NumberOrNest::Nest(v) => {
                        v[0]._explode_dive(side, value);
                    }
                }
            }
        }
    }

    fn try_explode(&mut self, depth: usize) -> (Option<u64>, Option<u64>, bool){
        match self {
            NumberOrNest::Number(_) => (None, None, false),
            NumberOrNest::Nest(children) => {
                if depth == 3 {
                    // explode logic
                    // assumes a depth 4 nesting can only have 1 Nest type child
                    if let NumberOrNest::Nest(to_explode) = &children[0] {

                        // get left and right child values
                        let left_number = match to_explode[0]{
                            NumberOrNest::Number(v) => v,
                            _ => panic!("Invalid structure")
                        };
                        let right_number = match to_explode[1]{
                            NumberOrNest::Number(v) => v,
                            _ => panic!("Invalid structure")
                        };
                        //println!("{:?}: Exploded {:?}", children, to_explode);
                        children[0] = NumberOrNest::Number(0); // set pair to 0
                        children[1]._explode_dive(ExplodeSide::Right, right_number); // increment right child by amount
                        (Some(left_number), None, true)
                    }else if let NumberOrNest::Nest(to_explode) = &children[1] {
                        let left_number = match to_explode[0]{
                            NumberOrNest::Number(v) => v,
                            _ => panic!("Invalid structure")
                        };
                        let right_number = match to_explode[1]{
                            NumberOrNest::Number(v) => v,
                            _ => panic!("Invalid structure")
                        };
                        //println!("{:?}: Exploded {:?}", children, to_explode);
                        children[0]._explode_dive(ExplodeSide::Left, left_number);
                        children[1] = NumberOrNest::Number(0);
                        (None, Some(right_number), true)
                    }else {
                        // Nothing
                        (None, None, false)
                    }
                    
                }else{
                    let left_explode_result = children[0].try_explode(depth+1);

                    if let Some(explode_value) = left_explode_result.1 {
                        children[1]._explode_dive(ExplodeSide::Right, explode_value);
                        return (left_explode_result.0, None, true);
                    }else if left_explode_result.2 {
                        return left_explode_result;
                    };

                    let right_explode_result = children[1].try_explode(depth+1);

                    if let Some(explode_value) = right_explode_result.0 {
                        children[0]._explode_dive(ExplodeSide::Left, explode_value);
                        return (None, right_explode_result.1, true);
                    }else if right_explode_result.2{
                        return right_explode_result;
                    };

                    (left_explode_result.0, right_explode_result.1, left_explode_result.2 || right_explode_result.2)
                }
            } 
        }
    
    }

    fn try_split(&mut self) -> bool{
        match self {
            NumberOrNest::Number(_) => false,
            NumberOrNest::Nest(children) => {
                let left_split = match &mut children[0] {
                    NumberOrNest::Number(n) => {
                        if *n >= 10 {
                            let half_n = *n as f32/2f32;
                            children[0] = NumberOrNest::Nest(vec!(NumberOrNest::Number(half_n.floor() as u64), NumberOrNest::Number(half_n.ceil() as u64)));
                            //println!("Splitting left {:?}", children);
                            true
                        }else{
                            false
                        }
                    },
                    NumberOrNest::Nest(_) => {
                        children[0].try_split()
                    }
                };
                left_split || match &mut children[1] {
                    NumberOrNest::Number(n) => {
                        if *n >= 10 {
                            let half_n = *n as f32/2f32;
                            children[1] = NumberOrNest::Nest(vec!(NumberOrNest::Number(half_n.floor() as u64), NumberOrNest::Number(half_n.ceil() as u64)));
                            //println!("Splitting right {:?}", children);
                            true
                        }else{
                            false
                        }
                    },
                    NumberOrNest::Nest(_) => {
                        children[1].try_split()
                    }
                }
            }
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            NumberOrNest::Number(n) => *n,
            NumberOrNest::Nest(n) => {
                3 * n[0].magnitude() + 2 * n[1].magnitude()
            }
        }
    }

    fn reduce(&mut self) {
        loop {
            let (x, y, exploded) = self.try_explode(0);
            if !exploded {
                let splitted = self.try_split();
                if !splitted{
                    break;
                }else{
                    //println!("split {}", self);
                }
            }else{
                //println!("explode {} {:?} {:?}", self, x, y);
            }
        }
    }
}

impl Display for NumberOrNest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            NumberOrNest::Number(n) => {
                write!(f, "{}", n)
            },
            NumberOrNest::Nest(n) => {
                write!(f, "[{}, {}]", n[0], n[1])
            }
        }
    }
}

fn main() {
    let input_file = File::open("input").expect("File not found");
    let input_lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let algebra = input_lines.iter().map(|l| serde_json::from_str(l).unwrap_or_else(|_| panic!("Line {} not parseable", l))).map(NumberOrNest::parse).reduce(|x, y| {
        println!("  {}\n+ {}", x, y);
        let mut composite = NumberOrNest::Nest(vec!(x, y));
        println!("~ {}", composite);
        composite.reduce();
        println!("= {}\n", composite);
        composite
    }).expect("Calculation error");
    println!("{} {}", algebra.magnitude(), algebra);
}
