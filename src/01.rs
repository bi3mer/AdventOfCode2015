use std::fs;

fn part2() {
    let file_name = "input/input01.txt";
    let content = fs::read_to_string(file_name)
        .expect("Unable to read file for some reason");

    let mut steps_taken = 0;
    let mut counter = 0;
    for c in content.chars() {
        steps_taken += 1;

        if c == '(' {
            counter += 1;
        } else {
            counter -= 1;
        }

        if counter == -1 {
            break;
        }
    }

    println!("Entered basement at: {}", steps_taken);
}

fn part1() {
    let file_name = "input/input01.txt";
    let content = fs::read_to_string(file_name)
        .expect("Unable to read file for some reason");

    let mut counter = 0;
    for c in content.chars() {
        if c == '(' {
            counter += 1;
        } else {
            counter -= 1;
        }
    }

    println!("Count: {}", counter);
}

fn main() {
    part1();
    part2();
}