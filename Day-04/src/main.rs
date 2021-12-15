use std::ffi::c_void;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, empty};
use std::ptr::null;

#[derive(Clone, Copy)]
struct Board {
    board: [i8; 25],
    marked: [bool; 25],
    turn: i32,
}

fn main() {
    let file = File::open("test.in").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let mut input: Vec<String> = reader
        .lines()
        //.map(|line| line.unwrap().parse::<instruction>().unwrap())
        .filter_map(io::Result::ok)
        .filter(|s| !s.eq(""))
        .collect();

    let mut i: i8 = 0;
    let mut boards: Vec<Board> = Vec::new();

    let numbers: Vec<i8> = input.swap_remove(0)
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i8>().unwrap())
        .collect();

    let mut all_boards: Vec<Board> = Vec::new();

    let mut i: i8 = 0;
    let mut temp_string: String = String::new();
    while !input.is_empty() {
        i += 1;
        temp_string += &input.swap_remove(0);
        temp_string += " ";
        if i == 5 {
            let mut temp: [i8; 25] = [0; 25];
            let mut n: usize = 0;

            for k in temp_string.trim()
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i8>().unwrap()) {
                temp[n] = k;
                n += 1;
            }
            let mut board = Board {
                board: temp,
                marked: [false; 25],
                turn: 0,
            };

            all_boards.push(board);

            temp_string = String::new();
            i = 0;
        }
    }

    for n in numbers {
        for b in &all_boards {
            play(*b, n);
        }
    }
    let mut low_board: Board;
    let mut low_turn: i32 = i32::MAX;
    let mut high_board: Board;
    let mut high_turn: i32 = i32::MIN;
    for b in all_boards {
        if b.turn < low_turn {
            low_turn = b.turn;
            low_board = b;
        }
        if b.turn > high_turn {
            high_turn = b.turn;
            high_board = b;
        }
    }

    dbg!(low_turn);
    dbg!(high_turn);
}

//fn get_score(marked: [bool; 25], nums: [i8; 25]) -> i32 {}

fn play(mut board: Board, n: i8) -> i32 {
    if !won(board.marked) { return board.turn; }
    for i in 0..25 {
        if board.board[i] == n {
            board.marked[i] = true;
            return board.turn + 1;
        }
    }
    return board.turn + 1;
}

fn won(marked: [bool; 25]) -> bool {
    let mut diag1: bool = true;
    let mut diag2: bool = true;
    for i in 0..5 {
        let mut hor: bool = true;
        let mut ver: bool = true;
        for j in 0..5 {
            hor = hor && marked[i * 5 + j];
            ver = ver && marked[i + j * 5];
        }
        if hor || ver { return true; }
        diag1 = diag1 && marked[6 * i];
        diag2 = diag2 && marked[4 + 4 * i];
    }
    return diag1 || diag2;
}
