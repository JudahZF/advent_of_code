use regex::Regex;
use std::{fs::File, io::{BufReader, prelude::*}};

fn check(lines: Vec<String>) -> i32 {
    let mut total: i32 = 0;
    let pattern = [
        [
            ['M', '*', 'S'],
            ['*', 'A', '*'],
            ['M', '*', 'S'],
        ],
        [
            ['S', '*', 'S'],
            ['*', 'A', '*'],
            ['M', '*', 'M'],
        ],
        [
            ['S', '*', 'M'],
            ['*', 'A', '*'],
            ['S', '*', 'M'],
        ],
        [
            ['M', '*', 'M'],
            ['*', 'A', '*'],
            ['S', '*', 'S'],
        ]
    ];
    let mut chars: Vec<Vec<char>> = vec![];
    for line in lines {
        let char_line: Vec<char> = line.chars().collect();
        chars.push(char_line);
    }
    for x in 0..chars.len()-2 {
        for y in 0..chars[x].len()-2 {
            for pat in pattern {
                if chars[x][y] != pat[0][0] {
                    continue;
                }
                if chars[x+2][y] != pat[2][0] {
                    continue;
                }
                if chars[x+1][y+1] != pat[1][1] {
                    continue;
                }
                if chars[x][y+2] != pat[0][2] {
                    continue;
                }
                if chars[x+2][y+2] != pat[2][2] {
                    continue;
                }
                println!("{}{}{}", chars[x][y], chars[x][y+1], chars[x][y+2]);
                println!("{}{}{}", chars[x+1][y], chars[x+1][y+1], chars[x+1][y+2]);
                println!("{}{}{}", chars[x+2][y], chars[x+2][y+1], chars[x+2][y+2]);
                println!("---");
                total += 1;
            }
        }
    }
    total
}

fn rotate(lines: Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<String> = vec![];
    let mut chars: Vec<Vec<char>> = vec![];
    let mut new_chars: Vec<Vec<char>> = vec![];
    for line in lines {
        let char_line: Vec<char> = line.chars().collect();
        chars.push(char_line);
    }
    for x in 0..chars.len() {
        let mut temp: Vec<char> = vec![];
        for _ in 0..chars[x].len() {
            temp.push('_');
        }
        new_chars.push(temp);
    }
    for x in 0..chars.len() {
        for y in 0..chars[x].len() {
            new_chars[chars[x].len() - y - 1][x] = chars[x][y]
        }
    }
    for line in new_chars {
        new_lines.push(String::from_iter(line))
    }
    new_lines
}

fn shift(lines: Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<String> = vec![];
    let mut chars: Vec<Vec<char>> = vec![];
    let mut new_chars: Vec<Vec<char>> = vec![];
    for line in lines {
        let char_line: Vec<char> = line.chars().collect();
        chars.push(char_line);
    }
    let total_height: usize = chars.len()+chars[0].len()+1;
    let max_width = chars[0].len();
    for _ in 0..total_height {
        let mut temp: Vec<char> = vec![];
        for _ in 0..max_width {
            temp.push('_');
        }
        new_chars.push(temp);
    }
    for x in 0..chars.len() {
        for y in 0..chars[x].len() {
            new_chars[x+y][y] = chars[x][y]
        }
    }
    for line in new_chars {
        new_lines.push(String::from_iter(line))
    }
    new_lines
}

fn main() {
    // Open the file and read it into a buffer
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = vec![];
    for line_raw in reader.lines() {
        let line_data = line_raw.unwrap();
        lines.push(line_data);
    }
    println!("Total: {}", check(lines.clone()))
}
