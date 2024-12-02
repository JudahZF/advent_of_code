use std::{fs::File, io::{BufReader, prelude::*}};

fn check_array(nums: Vec<i32>) -> bool {
    let mut safe = true;
    let mut last_diff = 0;
    for i in 1..nums.len() {
        let diff = &nums[i-1] - &nums[i];
        if last_diff != 0 {
            if (diff < 0)^(last_diff < 0) {
                safe = false;
                break;
            }
        }
        if !((1 <= diff.abs()) & (diff.abs() <= 3)) {
            safe = false;
            break;
        }
        last_diff = diff;
    }
    safe
}

fn main() {
    let mut total = 0;

    // Open the file and read it into a buffer
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_data = line.unwrap();
        let line_string = line_data.as_str();
        let line_items = line_string.split(" ").collect::<Vec<&str>>();
        let nums = line_items.into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let safe = check_array(nums.clone());
        if safe {
            total += 1;
        } else {
            for i in 0..nums.len() {
                let mut new_nums = nums.clone();
                new_nums.remove(i);
                let safe = check_array(new_nums);
                if safe {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("Total: {}", total);
}
