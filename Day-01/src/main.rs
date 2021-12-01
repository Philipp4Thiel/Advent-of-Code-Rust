use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("main.in").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    // totally not copied from googling "how to read ints" :kekw:

    let mut last = i32::MAX;
    let mut res = 0;

    for number in numbers.iter() {
        if *number > last {
            res += 1;
        }
        last = *number;
    }
    dbg!(res);

    last = i32::MAX;
    res = 0;

    for i in 2..numbers.len() {
        let cur = numbers[i] + numbers[i - 1] + numbers[i - 2];
        if cur > last {
            res += 1;
        }
        last = cur;
    }
    dbg!(res);
}