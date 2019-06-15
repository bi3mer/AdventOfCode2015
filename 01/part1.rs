use std::fs;

fn main() {
    let file_name = "input.txt";
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