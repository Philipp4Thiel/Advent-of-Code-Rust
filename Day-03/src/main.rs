use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("main.in").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let input: Vec<i32> = reader
        .lines()
        .map(|line| i32::from_str_radix(&*line.unwrap(), 2).unwrap())
        .collect();

    let mut gamma = 0;
    let mut eps = 0;
    let n = 12;
    for i in 0..n {
        let mut zeros = 0;
        let mut ones = 0;
        for cur in &input {
            let k = get_bit(cur, i);
            if k == 0 {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        if ones > zeros {
            gamma += 2_i32.checked_pow(i).unwrap();
        } else {
            eps += 2_i32.checked_pow(i).unwrap();
        }
    }
    println!("Task 1:");
    dbg!(gamma);
    dbg!(eps);
    dbg!(gamma * eps);

    let mut co2_vec = input.clone();
    let mut oxy_vec = input.clone();

    while co2_vec.len() > 1 {
        for i_1 in 0..n {
            let i = n - 1 - i_1;
            // get most common bit in vec
            let mut zeros = 0;
            let mut ones = 0;
            for cur in &co2_vec {
                let k = get_bit(cur, i);
                if k == 0 {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
            let bit = if ones >= zeros { 1 } else { 0 };
            let mut j = 0;
            while j < co2_vec.len() && co2_vec.len() > 1 {
                let cur = &co2_vec[j];
                if get_bit(cur, i) != bit {
                    co2_vec.remove(j);
                } else {
                    j += 1;
                }
            }
        }
    }

    while oxy_vec.len() > 1 {
        for i_1 in 0..n {
            let i = n - 1 - i_1;
            // get most common bit in vec
            let mut zeros = 0;
            let mut ones = 0;
            for cur in &oxy_vec {
                let k = get_bit(cur, i);
                if k == 0 {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
            let bit = if zeros > ones { 0 } else { 1 };
            let mut j = 0;
            while j < oxy_vec.len() && oxy_vec.len() > 1 {
                let cur = &oxy_vec[j];
                if get_bit(cur, i) == bit {
                    oxy_vec.remove(j);
                } else {
                    j += 1;
                }
            }
        }
    }

    let co2_rating = co2_vec[0];
    let oxy_rating = oxy_vec[0];

    println!("Task 2:");
    dbg!(co2_rating);
    dbg!(oxy_rating);
    dbg!(co2_rating * oxy_rating);
}

fn get_bit(n: &i32, i: u32) -> i32 {
    (n / 2_i32.checked_pow(i).unwrap()) % 2
}