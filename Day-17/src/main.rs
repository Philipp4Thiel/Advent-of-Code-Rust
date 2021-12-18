use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let input: String = BufReader::new(File::open("main.in")
        .expect("file wasn't found.")).lines().filter_map(Result::ok)
        .filter(|s| !s.eq("")).collect();

    let data = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)")
        .unwrap().captures(&input).unwrap();

    let area: [i32; 4] = [data[1].parse::<i32>().unwrap(),
        data[2].parse::<i32>().unwrap(),
        data[3].parse::<i32>().unwrap(),
        data[4].parse::<i32>().unwrap()];

    // find x so it halts in there

    let mut vel_x: i32 = 0;
    while vel_x * (vel_x + 1) / 2 < area[0] {
        vel_x += 1;
    }

    assert!(valid_x_cord(&area, vel_x * (vel_x + 1) / 2));
    //dbg!(vel_x);

    let mut best_y: i32 = 0;

    // try 10000 velocities for y
    for i in 0..1000 {
        let mut vel_y_temp = i;
        let mut vel_x_temp = vel_x;
        let mut pos_x = 0;
        let mut pos_y = 0;

        // first steps
        for _j in 0..vel_x {
            pos_x += vel_x_temp;
            pos_y += vel_y_temp;
            vel_y_temp -= 1;
            vel_x_temp = if vel_x_temp > 0 { vel_x_temp - 1 } else if vel_x_temp < 0 { vel_x_temp + 1 } else { 0 };
        }
        while pos_y > max(area[2], area[3]) {
            pos_x += vel_x_temp;
            pos_y += vel_y_temp;
            vel_y_temp -= 1;
            vel_x_temp = if vel_x_temp > 0 { vel_x_temp - 1 } else if vel_x_temp < 0 { vel_x_temp + 1 } else { 0 };
        }
        if valid_cord(&area, pos_x, pos_y) {
            best_y = i;
            //dbg!(best_y);
        }
    }
    println!("Task 1: {} with velocity:({}, {})", (best_y * (best_y + 1) / 2), vel_x, best_y);

    let mut counter = 0;
    for vel_y in min(area[2], area[3])..1000 {
        for vel_x in 0..1000 {
            if will_hit(&area, vel_x, vel_y) {
                counter += 1;
            }
        }
    }
    println!("Task 2: {}", counter);
}

fn will_hit(area: &[i32; 4], mut vel_x: i32, mut vel_y: i32) -> bool {
    assert!(vel_x >= 0);
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while y >= min(area[2], area[3]) {
        x += vel_x;
        y += vel_y;
        vel_y -= 1;
        vel_x = if vel_x > 0 { vel_x - 1 } else { 0 };
        if valid_cord(area, x, y) { return true; }
    }
    return false;
}

fn valid_cord(area: &[i32; 4], x: i32, y: i32) -> bool {
    return valid_x_cord(area, x) && valid_y_cord(area, y);
}

fn valid_x_cord(area: &[i32; 4], x: i32) -> bool {
    return x >= min(area[0], area[1]) && x <= max(area[0], area[1]);
}

fn valid_y_cord(area: &[i32; 4], y: i32) -> bool {
    return y >= min(area[2], area[3]) && y <= max(area[2], area[3]);
}
