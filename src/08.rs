use std::env;
use std::fs;

fn part1(string: &str) -> u16 {
    let mut size = 0;
    let mut index = 1;

    loop {
        if index >= string.len() - 1{
            break;
        }

        size += 1;
        
        let character = string.chars().nth(index).unwrap();
        if character == '\\'  {
            let next_character = string.chars().nth(index + 1).unwrap();
            if next_character == 'x' {
                index += 3;
            } else {
                index += 1;
            }
        }

        index += 1;
    }
    
    (string.len() as u16) - size
}

fn part2(string: &str) -> u16 {
    let mut size = 0;
    for character in string.chars() {
        if character == '\\' {
            size += 2;
        } else if character == '"' {
            size += 2
        } else {
            size += 1;
        }
    }
    
    (size + 2) - (string.len() as u16)
}

fn main() {
    let file = format!("{}/input/input08.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file)
        .expect("Something went wrong reading the file.");
    
    let lines = content.split("\n");
    let mut part1_count = 0;
    let mut part2_count = 0;

    for line in lines {
        part1_count += part1(line);
        part2_count += part2(line);
    }

    println!("Part 1: {}", part1_count);
    println!("Part 2: {}", part2_count);
}