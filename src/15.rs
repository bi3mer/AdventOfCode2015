use std::collections::HashMap;
use std::env;
use std::cmp;
use std::fs;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let file = format!("{}/input/input15.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let re = Regex::new(r"(\D+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    let mut name_to_data: HashMap<String, Vec<i32>> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();

    for line in content.split("\n") {
        let capture = re.captures(line).unwrap();

        keys.push(capture[1].to_string());
        name_to_data.insert(
            capture[1].to_string(),
            vec![
                capture[2].parse().unwrap(),
                capture[3].parse().unwrap(),
                capture[4].parse().unwrap(),
                capture[5].parse().unwrap(),
                capture[6].parse().unwrap()
            ]);
    }

    let max_teaspoons = 100;
    let mut best_score: i32 = 0;
    let mut best_cal_score: i32 = 0;
    let num_keys = keys.len();

    for c in (0..101).permutations(num_keys) {
        if c.iter().sum::<i32>() == max_teaspoons {
            let mut capacity: i32 = 0;
            let mut durability: i32 = 0;
            let mut flavor: i32 = 0;
            let mut texture: i32 = 0;
            let mut calories: i32 = 0;

            for key_index in 0..num_keys {
                let key = keys.get(key_index).unwrap();
                let data = name_to_data.get(key).unwrap();
                
                capacity += data[0] * c[key_index];
                durability += data[1] * c[key_index];
                flavor += data[2] * c[key_index];
                texture += data[3] * c[key_index];
                calories += data[4] * c[key_index];
            }

            let total = cmp::max(0, capacity) *
                        cmp::max(0, durability) *
                        cmp::max(0, flavor) * 
                        cmp::max(0, texture);

            if calories == 500 && total > best_cal_score {
                best_cal_score = total;
            }

            if total > best_score {
                best_score = total;
            }
        }
    }

    println!("Part1: {}", best_score);
    println!("Part2: {}", best_cal_score);

}