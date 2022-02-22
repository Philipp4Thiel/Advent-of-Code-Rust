use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let input: Vec<String> = BufReader::new(File::open("test.in")
        .expect("file wasn't found.")).lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
        .collect();

    let scanner = Regex::new(r"-{3} scanner (\d+) -{3}").unwrap();
    let coord = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap();

    let mut scanners: Vec<Vec<Coord>> = Vec::new();
    let mut cur: usize = 0;

    for line in input.iter() {
        if scanner.is_match(line) {
            let temp = &scanner.captures(line).unwrap()[1];
            cur = temp.parse::<usize>().unwrap();
            scanners.push(Vec::new());
            continue;
        }

        if coord.is_match(line) {
            let temp = &coord.captures(line).unwrap();

            scanners[cur].push(Coord { x: temp[1].parse::<i32>().unwrap(), y: temp[2].parse::<i32>().unwrap(), z: temp[3].parse::<i32>().unwrap() });
            continue;
        }
        panic!()
    }

    let mut distances: Vec<HashSet<i32>> = Vec::new();

    for cur in 0..scanners.len() {
        distances.push(HashSet::new());
        for a in 0..scanners[cur].len() - 1 {
            for b in a + 1..scanners[cur].len() {
                let a = &scanners[cur][a];
                let b = &scanners[cur][b];

                let dx = a.x - b.x;
                let dy = a.y - b.y;
                let dz = a.z - b.z;

                let d = dx * dx + dy * dy + dz * dz;

                if distances[cur].contains(&d) { panic!(); }
                distances[cur].insert(d);
            }
        }
    }
    while distances.len() > 1 {
        let mut new_distances: Vec<HashSet<i32>> = Vec::new();
        let mut new_scanners: Vec<Vec<Coord>> = Vec::new();
        let mut processed: Vec<usize> = Vec::new();

        for i in 0..distances.len() - 1 {
            for j in i + 1..distances.len() {
                if check_overlap(&distances[i], &distances[j]) {
                    println!("Scanner {} overlaps with Scanner {}", i, j);

                    // TODO merge scanner[i] and scanner[j] and push result onto new_scanners
                    //  calculate distances for result

                    if !processed.contains(&i) { processed.push(i); }
                    if !processed.contains(&j) { processed.push(j); }
                }
            }
        }

        for i in 0..distances.len() - 1 {
            if !processed.contains(&i) {
                processed.push(i);

                new_distances.push(distances[i].clone());
                new_scanners.push(scanners[i].clone());
            }
        }

        distances = new_distances;
        scanners = new_scanners;
    }

    println!("Part 1: {}", distances.len());
}

/**
 * find overlaps by checking if two sets have 66 or more of the same distance
 * (i have no clue if that works or not but :prayge:)
 */
fn check_overlap(a: &HashSet<i32>, b: &HashSet<i32>) -> bool {
    let mut counter: u32 = 0;
    for i in a {
        for j in b {
            if i == j { counter += 1; }
        }
    }

    return counter >= 66;
}
