use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;

fn main() {
    ///////// Input and Setup
    // let file = format!("{}/input/input19_test.txt", env::current_dir().unwrap().to_str().unwrap());
    let file = format!("{}/input/input19.txt", env::current_dir().unwrap().to_str().unwrap());

    let content = fs::read_to_string(file).expect("Something went wrong reading the file");
    
    let mut molecule_to_outputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut outputs_to_molecules: Vec<(&str, &str)> = Vec::new();
    let mut input = "";

    for line in content.split("\n") {
        if line.contains(" => ") {
            let split: Vec<&str>  = line.split(" => ").collect();

            match molecule_to_outputs.get_mut(split[0]) {
                Some(v) => {
                    v.push(split[1])
                },
                _ => {
                    molecule_to_outputs.insert(split[0], vec![split[1]]);
                }
            }

            outputs_to_molecules.push((split[1], split[0]));
        } else {
            input = line;
        }
    }

    ///////// Part 1
    let mut combinations: HashSet<String> = HashSet::new();
    for (key, replacements) in molecule_to_outputs {
        let re = Regex::new(key).unwrap();
        for m in re.find_iter(input) {
            let start = m.start();

            for new_replacement in &replacements {
                let mut copy = input.clone().to_string();
                for _ in m.start()..m.end() {
                    copy.remove(start);
                }

                copy.insert_str(m.start(), new_replacement.clone());
                combinations.insert(copy);
            }
        }
    }

    println!("possible outcomes: {}", combinations.len());

    ///////// Part 2
    // https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4cu5b?utm_source=share&utm_medium=web2x
    let mut target = input.to_string();
    let mut steps_taken = 0;
    let mut rng = thread_rng();

    while target != "e" {
        let tmp = target.clone();
        for (key, replacement) in &outputs_to_molecules {
            if target.contains(key) {
                target = target.replacen(key, replacement, 1);
                steps_taken += 1;
                break;
            }
        } 

        if tmp == target {
            target = input.to_string();
            steps_taken = 0;
            outputs_to_molecules.shuffle(&mut rng);
        }
    }

    println!("Steps taken: {}", steps_taken);
}