use std::io::{BufRead, BufReader};
use std::fs::File;

fn get_numbers(input: &str) -> (usize, usize) {
    let mut split = input.split(",");
    let input_one = split.next().unwrap();
    let input_two = split.next().unwrap();

    (input_one.parse::<usize>().unwrap(), input_two.parse::<usize>().unwrap())
}

fn main() {
    let mut grid1 = [[false; 1000]; 1000];
    let mut grid2 = [[0; 1000]; 1000];

    let file_name = "input/input06.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut split_line = line.split("through");

        let bad_incomplete_start = split_line.next().unwrap();
        let mut split_bad_start = bad_incomplete_start.split(" ");
        let incomplete_end = split_line.next().unwrap();
        
        if line.starts_with("turn on") {
            split_bad_start.next();
            split_bad_start.next();
        } else if line.starts_with("turn off") {
            split_bad_start.next();
            split_bad_start.next();
        } else if line.starts_with("toggle") {
            split_bad_start.next();
        } else {
            println!("uh oh, '{}' not recognized", line);
        }

        let incomplete_start = split_bad_start.next().unwrap();

        let (start_x, start_y) = get_numbers(incomplete_start.trim());
        let (end_x, end_y) = get_numbers(incomplete_end.trim());

        // I know... duplicate code
        for y in start_y..end_y+1 {
            for x in start_x..end_x+1 {
                if line.starts_with("turn on") {
                    grid1[y][x] = true;
                    grid2[y][x] += 1;
                } else if line.starts_with("turn off") {
                    grid1[y][x] = false;

                    if grid2[y][x] > 0 {
                        grid2[y][x] -= 1;
                    }
                } else {
                    grid1[y][x] = !grid1[y][x];
                    grid2[y][x] += 2;
                }
            }
        }
    }

    let mut lights_on = 0;
    let mut brightness = 0;

    for y in 0..1000 {
        for x in 0..1000 {
            if grid1[y][x] {
                lights_on += 1;
            }

            brightness += grid2[y][x];
        }
    }
    println!("Lights on:  {}", lights_on);
    println!("Brightness: {}", brightness);
}