use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct BingoBoard{
    board: [[u32; 5]; 5],
    pulled: [[bool; 5]; 5]
}

impl BingoBoard {
    fn get_score(&self) -> u32 {
        let mut score = 0;

        for y in 0..5 {
            for x in 0.. 5{
                if !self.pulled[y][x] {
                    score += self.board[y][x];
                }
            }
        }

        score
    }
}

fn find_score(pulls: &[u32], number_contained_in: HashMap<u32, Vec<usize>>, boards: &mut[BingoBoard])-> u32 {
    let mut boards_won = HashSet::new();

    for i in 0..boards.len(){
        boards_won.insert(i);
    }

    for pull in pulls {
        match number_contained_in.get(&pull) {
            Some(board_indexes) => {
                for board_index in board_indexes {
                    if boards_won.contains(board_index){
                        let mut board = &mut boards[*board_index];

                        for y in 0..5 {
                            for x in 0..5{
                                if board.board[y][x] == *pull {
                                    board.pulled[y][x] = true;
                                }
                            }
                        }

                        for y in 0..5 {
                            let mut row_win = true;
                            for x in 0..5{
                                row_win = row_win && board.pulled[y][x];
                            }

                            if row_win {
                                if boards_won.len() == 1 {
                                    return board.get_score() * pull;
                                }else{
                                    boards_won.remove(board_index);
                                }
                            }
                        }

                        for x in 0.. 5{
                            let mut column_win = true;
                            for y in 0..5{
                                column_win = column_win && board.pulled[y][x];
                            }

                            if column_win {
                                if boards_won.len() == 1 {
                                    return board.get_score() * pull;
                                }else{
                                    boards_won.remove(board_index);
                                }
                            }
                        }
                    }
                }
            },
            None => {}
        }
    }
    0
}

fn main() {
    let input_file = File::open("input").expect("File not found");
    let mut lines = io::BufReader::new(input_file).lines().map(|x| x.unwrap());

    let pulls:Vec<u32> = lines.next().expect("Invalid file format").split(",").map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

    let mut boards = Vec::new();
    let mut number_contained_in = HashMap::new();

    while lines.next().is_some() {
        let mut new_board = BingoBoard{board: [[0u32; 5]; 5], pulled: [[false; 5]; 5]};

        for y in 0 .. 5 {
            let line: Vec<u32> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
            for (x, val) in line.iter().enumerate() {
                new_board.board[y][x] = *val;

                if !number_contained_in.contains_key(val){
                    number_contained_in.insert(*val, Vec::new());
                }

                number_contained_in.get_mut(val).unwrap().push(boards.len());
            }
        }
        boards.push(new_board);
    }

    let score = find_score(&pulls, number_contained_in, &mut boards);

    println!("{}", score);
}

