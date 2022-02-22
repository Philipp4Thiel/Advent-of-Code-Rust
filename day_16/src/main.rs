use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;

fn main() {
    let file = File::open("main.in").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let mut input: String = reader.lines().filter_map(Result::ok)
        .filter(|s| !s.eq("")).collect();

    input = input.replace("0", "0000");
    input = input.replace("1", "0001");
    input = input.replace("2", "0010");
    input = input.replace("3", "0011");
    input = input.replace("4", "0100");
    input = input.replace("5", "0101");
    input = input.replace("6", "0110");
    input = input.replace("7", "0111");
    input = input.replace("8", "1000");
    input = input.replace("9", "1001");
    input = input.replace("A", "1010");
    input = input.replace("B", "1011");
    input = input.replace("C", "1100");
    input = input.replace("D", "1101");
    input = input.replace("E", "1110");
    input = input.replace("F", "1111");

    let mut versions: Vec<u32> = Vec::new();
    let mut stack: Vec<u128> = Vec::new();

    parse_package(&input, 0, &mut versions, &mut stack);
    println!("T1: {}", u32::sum(versions.iter()));
    println!("T2: {}", stack.pop().unwrap());
}

fn bit_to_int(bit_string: &str) -> u32 {
    return u32::from_str_radix(bit_string, 2).unwrap();
}

fn parse_package(package: &str, mut pos: usize, versions: &mut Vec<u32>, stack: &mut Vec<u128>) -> usize {
    let version: u32 = bit_to_int(&package[pos..pos + 3]);
    versions.push(version);
    pos += 3;
    let package_type: u32 = bit_to_int(&package[pos..pos + 3]);
    pos += 3;

    if package_type == 4 {
        let mut value: u128 = 0;
        let mut temp: u32 = bit_to_int(&package[pos..pos + 1]);
        pos += 1;
        loop {
            value = (value * 16) + bit_to_int(&package[pos..pos + 4]) as u128;
            pos += 4;
            if temp == 0 { break; }
            temp = bit_to_int(&package[pos..pos + 1]);
            pos += 1;
        }
        //println!("Package v{} is a literal package with value {}", version, value);
        stack.push(value);
        return pos;
    }
    let mut subpackages: u32 = 0;
    let subpackage_type: u32 = bit_to_int(&package[pos..pos + 1]);
    pos += 1;
    if subpackage_type == 1 {
        let num_packages = bit_to_int(&package[pos..pos + 11]);
        pos += 11;
        //println!("Package v{} contains {} subpackages and is of type {}", version, subpackages, package_type);
        for _i in 0..num_packages {
            pos = parse_package(&package, pos, versions, stack);
            subpackages += 1;
        }
    } else if subpackage_type == 0 {
        let num_bits = bit_to_int(&package[pos..pos + 15]);
        pos += 15;
        //println!("Package v{} contains {} bits of subpackages and is of type {}", version, subpackages, package_type);
        let pos_start: usize = pos;
        while pos < pos_start + num_bits as usize {
            pos = parse_package(&package, pos, versions, stack);
            subpackages += 1;
        }
    } else {
        panic!("wtf");
    }

    match package_type {
        0 => {
            let mut res: u128 = 0;
            for _i in 0..subpackages as usize {
                res += stack.pop().unwrap();
            }
            stack.push(res);
        } // +
        1 => {
            let mut res: u128 = 1;
            for _i in 0..subpackages as usize {
                res *= stack.pop().unwrap();
            }
            stack.push(res);
        } // *
        2 => {
            let mut res: u128 = u128::MAX;
            for _i in 0..subpackages as usize {
                res = min(stack.pop().unwrap(), res);
            }
            stack.push(res);
        } // min
        3 => {
            let mut res: u128 = u128::MIN;
            for _i in 0..subpackages as usize {
                res = max(stack.pop().unwrap(), res);
            }
            stack.push(res);
        } // max
        5 => {
            let second: u128 = stack.pop().unwrap();
            let first: u128 = stack.pop().unwrap();
            stack.push(if first > second { 1 } else { 0 });
        } // >
        6 => {
            let second: u128 = stack.pop().unwrap();
            let first: u128 = stack.pop().unwrap();
            stack.push(if first < second { 1 } else { 0 });
        } // <
        7 => {
            let first: u128 = stack.pop().unwrap();
            let second: u128 = stack.pop().unwrap();
            stack.push(if first == second { 1 } else { 0 });
        } // =
        _ => { panic!("unknown package type"); }
    }
    return pos;
}