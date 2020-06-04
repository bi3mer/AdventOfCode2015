use std::fs;
use std::env;

fn get_length(string: &str) -> u16 {
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
    
    println!("{} - {} = {}", string.len(), size, (string.len() as u16) - size);
    (string.len() as u16) - size
}

fn main() {
    let file = format!("{}/input/input08.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file)
        .expect("Something went wrong reading the file.");
    
    let lines = content.split("\n");
    let mut count = 0;

    for line in lines {
        count += get_length(line);
    }

    println!("Part 1: {}", count);
}