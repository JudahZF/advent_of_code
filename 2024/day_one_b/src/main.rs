use std::{fs::File, io::{BufReader, prelude::*}};

struct Lists {
    list_a: Vec<i32>,
    list_b: Vec<i32>
}

fn read_file() -> Result<Lists, &'static str> {
    // Open the file and read it into a buffer
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Create the lists
    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];

    for line in reader.lines() {
        // Get the line as a string
        let line_data = line.unwrap();
        let line_string = line_data.as_str();

        // Split the line into list a and list b
        let line_items = line_string.split("   ").collect::<Vec<&str>>();
        list_a.push(line_items[0].parse::<i32>().unwrap());
        list_b.push(line_items[1].parse::<i32>().unwrap());
    }

    // Return the lists
    Ok(Lists {list_a, list_b})
}

fn main() {
    // Read the lists from the file
    let lists = read_file().unwrap();
    let mut list_a = lists.list_a;
    let mut list_b = lists.list_b;

    // Sort the lists
    list_a.sort();
    list_b.sort();

    // Add up the total
    let mut total = 0;
    for i in list_a {
        let mut times_seen = 0;
        for j in &list_b {
            if &i == j {
                times_seen += 1;
            }
        }

        total+= i * times_seen;
    }

    // Print the answer
    println!("Total: {}", total)
}
