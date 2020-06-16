use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    // let file = format!("{}/input/input17_test.txt", env::current_dir().unwrap().to_str().unwrap());
    // let expected_total = 25;
    
    let file = format!("{}/input/input17.txt", env::current_dir().unwrap().to_str().unwrap());
    let expected_total = 150;

    let content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut bins:Vec<u16> = Vec::new();

    for line in content.split("\n") {
        bins.push(line.parse::<u16>().unwrap());
    }

    let mut min_container_index = bins.len() + 1;
    let mut min_container_combinations = 0;
    let mut combinations_found = 0;

    for bin_size in 2..bins.len()+1 {
        for combination in bins.iter().combinations(bin_size) {
            let mut sum: u16 = 0;
            for value in combination.iter() {
                sum += *value;
            }

            if sum == expected_total {
                combinations_found += 1;

                if bin_size <= min_container_index  {
                    min_container_index = bin_size;
                    min_container_combinations += 1;
                }
            }
        }
    }

    println!("Num Combinations Found: {}", combinations_found);
    println!("Combinations For Min:   {}", min_container_combinations)
}