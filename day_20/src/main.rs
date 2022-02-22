use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coords(i32, i32);

fn main() {
    let mut input: Vec<String> = BufReader::new(File::open("main.in")
        .expect("file wasn't found.")).lines()
        .map(|s| s.unwrap())
        .filter(|s| !s.is_empty())
        .collect();

    let code = input.remove(0);

    let mut image: HashSet<Coords> = HashSet::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            match input[y].as_bytes()[x] {
                b'#' => { image.insert(Coords(x as i32, y as i32)); }
                _ => {}
            }
        }
    }

    for i in 0..50 {
        let mut new_image: HashSet<Coords> = HashSet::new();

        let range = _get_range(&image);

        for y in range.2 - 3..range.3 + 3 {
            for x in range.0 - 3..range.1 + 3 {
                if enhance(x, y, &image, &code) {
                    new_image.insert(Coords(x, y));
                }
            }
        }
        if i % 2 == 1 {
            image = new_image.into_iter().filter(|Coords(x, y)|
                *x > (0 - (i + 2))
                    && *x < (100 + (i + 2))
                    && *y > (0 - (i + 2))
                    && *y < (100 + (i + 2))
            ).collect();
        } else {
            image = new_image;
        }
        if i == 1 {
            println!("Part 1: {}", count(&image, i + 2));
        }
    }
    println!("Part 2: {}", count(&image, 51));
}

fn count(image: &HashSet<Coords>, i: i32) -> i32 {
    return image.clone().into_iter().filter(|Coords(x, y)|
        *x > (0 - i)
            && *x < (100 + i)
            && *y > (0 - i)
            && *y < (100 + i)
    ).count() as i32;
}

fn _print_image(image: &HashSet<Coords>) -> () {
    let range = _get_range(image);
    for y in range.2..range.3 {
        for x in range.0..range.1 {
            match image.contains(&Coords(x, y)) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!("");
    }
    println!("");
}

fn _get_range(image: &HashSet<Coords>) -> (i32, i32, i32, i32) {
    let mut max_x: i32 = i32::MIN;
    let mut min_x: i32 = i32::MAX;
    let mut max_y: i32 = i32::MIN;
    let mut min_y: i32 = i32::MAX;

    for coords in image {
        max_x = max(max_x, coords.0);
        min_x = min(min_x, coords.0);
        max_y = max(max_y, coords.1);
        min_y = min(min_y, coords.1);
    }

    return (min_x - 1, max_x + 2, min_y - 1, max_y + 2);
}

fn enhance(x: i32, y: i32, image: &HashSet<Coords>, code: &String) -> bool {
    let mut pow = 9;
    let mut res: usize = 0;
    for y in (y - 1)..(y + 2) {
        for x in (x - 1)..(x + 2) {
            pow -= 1;
            if image.contains(&Coords(x, y)) {
                res += 2_i32.pow(pow) as usize;
            }
        }
    }

    match code.as_bytes()[res] {
        b'.' => return false,
        b'#' => return true,
        _ => panic!()
    }
}
