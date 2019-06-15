use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_space(l: u32, w: u32, h: u32) -> u32 {
    let lw = l * w;
    let lh = l * h;
    let wh = w * h;

    let smallest;
    if lw < lh && lw < wh {
        smallest = lw;
    } else if lh < wh {
        smallest = lh;
    } else {
        smallest = wh;
    }

    (2 * lw) + (2* lh) + (2 * wh) + smallest
}

fn calculate_ribbons(l: u32, w: u32, h: u32) -> u32{
    let lw = l * w;
    let lh = l * h;
    let wh = w * h;

    let smallest_perimeter;
    if lw < lh && lw < wh {
        smallest_perimeter = 2*l + 2*w;
    } else if lh < wh {
        smallest_perimeter = 2*l + 2*h;
    } else {
        smallest_perimeter = 2*w + 2*h;
    }

    l*w*h + smallest_perimeter
}

fn main() {
    let file_name = "input.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut total_cost = 0;
    let mut total_ribbons = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut split : Vec<&str> = line.split("x").collect();

        let l = split[0].parse::<u32>().unwrap();
        let w = split[1].parse::<u32>().unwrap();
        let h = split[2].parse::<u32>().unwrap();

        total_cost += calculate_space(l, w, h);
        total_ribbons += calculate_ribbons(l, w, h);
    }
    
    println!("Case 1 passed: {}", calculate_space(2,3,4) == 58);
    println!("Case 2 passed: {}", calculate_space(1,1,10) == 43);
    println!("Case 3 passed: {}", calculate_ribbons(2,3,4) == 34);
    println!("Case 4 passed: {}", calculate_ribbons(1,1,10) == 14);

    println!("Total Cost: {}", total_cost);
    println!("Total Ribbons: {}", total_ribbons);
}