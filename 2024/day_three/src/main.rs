use regex::Regex;
use std::{fs::File, io::{BufReader, prelude::*}};

#[derive(Debug)]
struct Maybe {
    start: i32,
    state: bool
}

fn main() {
    let mut total = 0;

    // Open the file and read it into a buffer
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)").unwrap();
    let can = Regex::new(r"do\(\)").unwrap();
    let cannot = Regex::new(r"don't\(\)").unwrap();
    // let mut results = vec![];
    let mut string: String = "".to_owned();
    for line_raw in reader.lines() {
        let line_data = line_raw.unwrap();
        string.push_str(line_data.as_str());
    }
    let mut allowed: Vec<Maybe> = vec![];
    for mat in can.find_iter(&string) {
        let result = &mat;
        allowed.push(Maybe {start: result.start().try_into().unwrap(), state: true});
    }
    for mat in cannot.find_iter(&string) {
        let result = &mat;
        allowed.push(Maybe {start: result.start().try_into().unwrap(), state: false});
    }
    allowed.sort_by(|a, b| a.start.cmp(&b.start));
    let mut index_to_remove: Vec<usize> = vec![];
    for i in 1..allowed.len() {
        if allowed[i].state == allowed[i-1].state {
            index_to_remove.push(i);
        }
    }
    for i in index_to_remove.into_iter().rev() {
        allowed.remove(i);
    }
    println!("{:?}", allowed);
    allowed[0].start = 0;
    let mut allowed_range: Vec<usize> = vec![];
    for i in 1..allowed.len() {
        if allowed[i].state == false {
            let range = allowed[i-1].start..=allowed[i].start;
            println!("Allowed Range: {} -> {}", allowed[i-1].start, allowed[i].start);
            for j in range {
                allowed_range.push(j.try_into().unwrap());
            }
        }
    }
    for mat in re.find_iter(&string) {
        let result = mat;
        let start = result.start();
        if allowed_range.contains(&start) {
            let result = result.as_str();
            let result = &result[4..];
            let result = &result[..result.len() - 1];
            let result = result.split(',');
            let result = result.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let sum = result[0]*result[1];
            total += sum;
        }
    }
    println!("{}", total);
}
