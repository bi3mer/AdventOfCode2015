use std::env;
use std::fs;

// NOTES:
// - two registers a and b
// - registers hold non-negative integer
// - registers start at the value of 0

fn main() {
    // let file = format!("{}/input/input23_test.txt", env::current_dir().unwrap().to_str().unwrap());
    // let content = fs::read_to_string(file).expect("Something went wrong reading the file");

    let file = format!("{}/input/input23.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut a: i64 = 0;
    let mut b: i64 = 0;

    // set to false if part 1
    if true {
        a += 1;
    }

    let mut commands: Vec<Vec<&str>> = Vec::new();

    for line in content.split("\n") {
        let split_line: Vec<&str> = line.split(" ").collect();
        commands.push(split_line);
    }
        
    let num_commands = commands.len();
    let mut command_index: usize = 0;

    while command_index < num_commands {
        let command = &commands[command_index];
        match command[0] {
            "hlf" => {
                command_index += 1;
                if command[1] == "a" {
                    a /= 2;
                } else {
                    b /= 2;
                }
            },
            "tpl" => {
                command_index += 1;
                if command[1] == "a" {
                    a *= 3;
                } else {
                    b *= 3;
                }
            },
            "inc" => {
                command_index += 1;
                if command[1] == "a" {
                    a += 1;
                } else {
                    b += 1;
                }
            },
            "jmp" => {
                let offset = command[1].parse::<i64>().unwrap();
                command_index = ((command_index as isize) + (offset as isize)) as usize;
            },
            "jie" => {
                let offset = command[2].parse::<i64>().unwrap();
                let register_name = command[1].replace(",", "");
                
                if register_name == "a" && a % 2 == 0 {
                    command_index = ((command_index as isize) + (offset as isize)) as usize;
                } else if register_name == "b" && b % 2 == 0 {
                    command_index = ((command_index as isize) + (offset as isize)) as usize;
                } else {
                    command_index += 1;
                }
            },
            "jio" => {
                let offset = command[2].parse::<i64>().unwrap();
                let register_name = command[1].replace(",", "");
                
                if register_name == "a" && a == 1 {
                    command_index = ((command_index as isize) + (offset as isize)) as usize;
                } else if register_name == "b" && b == 1 {
                    command_index = ((command_index as isize) + (offset as isize)) as usize;
                } else {
                    command_index += 1;
                }
            },
            _ => {
                println!("{} not handled.", command[0]);
                break;
            }
        }
    }

    println!("a = {}", a);
    println!("b = {}", b);
}