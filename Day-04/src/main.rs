use std::fs::File;
use std::io::{BufRead, BufReader};

const NUM_BOARDS: usize = 100;
// 3 for testing 100 for actual test

fn main() {
    let file = File::open("main.in").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let mut input: Vec<String> = reader
        .lines()
        //.map(|line| line.unwrap().parse::<instruction>().unwrap())
        .filter_map(Result::ok)
        .filter(|s| !s.eq(""))
        .collect();

    let numbers: Vec<u8> = input.swap_remove(0).split(",").map(|s| s.trim())
        .filter(|s| !s.is_empty()).map(|s| s.parse::<u8>().unwrap()).collect();

    let mut boards: [[[u8; 5]; 5]; NUM_BOARDS] = [[[0; 5]; 5]; NUM_BOARDS];
    let mut marked: [[[bool; 5]; 5]; NUM_BOARDS] = [[[false; 5]; 5]; NUM_BOARDS];

    for board in 0..NUM_BOARDS {
        for i in 0..5 {
            let cur_numbers: Vec<u8> = input.swap_remove(0).trim().split(" ")
                .map(|s| s.trim()).filter(|s| !s.is_empty())
                .map(|s| s.parse::<u8>().unwrap()).collect();
            for j in 0..5 {
                boards[NUM_BOARDS - 1 - board][4 - i][j] = cur_numbers[j];
            }
        }
    }

    let mut task1 = false;
    let mut win_counter: u8 = 0;
    let mut done: [bool; NUM_BOARDS] = [false; NUM_BOARDS];
    for number in numbers.clone() {
        // play move on all boards
        for b in 0..NUM_BOARDS {
            for i in 0..5 {
                for j in 0..5 {
                    if boards[b][i][j] == number {
                        marked[b][i][j] = true;
                    }
                }
            }
        }

        // check for win
        for b in 0..NUM_BOARDS {
            if !done[b] && won(&marked[b]) {
                done[b] = true;
                win_counter += 1;
                if win_counter == NUM_BOARDS as u8 {
                    println!("Task 2: {}", get_score(&boards[b], &marked[b]) * (number as u32));
                }
                if !task1 {
                    println!("Task 1: {}", get_score(&boards[b], &marked[b]) * (number as u32));
                    task1 = true;
                }
            }
        }
    }
}

fn get_score(board: &[[u8; 5]; 5], marked: &[[bool; 5]; 5]) -> u32 {
    let mut unmarked: u32 = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !marked[i][j] {
                unmarked += board[i][j] as u32;
            }
        }
    }
    return unmarked;
}

fn won(marked: &[[bool; 5]; 5]) -> bool {
    for i in 0..5 {
        let mut hor: bool = true;
        let mut ver: bool = true;
        for j in 0..5 {
            hor = hor && marked[i][j];
            ver = ver && marked[j][i];
        }
        if hor || ver { return true; }
    }
    return false;
}
