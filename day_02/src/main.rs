use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;

struct instruction {
    dir: String,
    x: i32,
}

impl FromStr for instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited: Vec<&str> = s
            .trim()
            .split(' ')
            .collect();
        Ok(instruction { dir: splited[0].parse::<String>()?, x: splited[1].parse::<i32>().unwrap() })
    }
}

fn main() {
    let file = File::open("main.in").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let instructions: Vec<instruction> = reader
        .lines()
        .map(|line| line.unwrap().parse::<instruction>().unwrap())
        .collect();

    let mut dist = 0;
    let mut depth = 0;
    let mut aim = 0;

    for ins in instructions {
        match ins.dir.as_str() {
            "forward" => {
                dist += ins.x;
                depth += aim * ins.x;
            }
            "down" => aim += ins.x,
            "up" => aim -= ins.x,
            _ => println!("{}", ins.dir.as_str())
        }
    }
    dbg!(dist * depth);
}