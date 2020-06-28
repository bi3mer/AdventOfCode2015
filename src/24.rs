use itertools::Itertools;
use std::cmp;
use std::env;
use std::fs;

fn main() {
    // let file = format!("{}/input/input24_test.txt", env::current_dir().unwrap().to_str().unwrap());
    // let content = fs::read_to_string(file).expect("Something went wrong reading the file");

    let file = format!("{}/input/input24.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut weights: Vec<i64> = Vec::new();

    for line in content.split("\n") {
        weights.push(line.parse::<i64>().unwrap());
    }
    
    let num_compartments: i64 = 4; // 3 for part 1, 4 for part 2
    let expected_weight: i64 = weights.iter().sum::<i64>() / num_compartments;
    let mut lowest_quantum_entanglement: i64 = 1000000000000;
    let mut should_break = false;

    for i in 1..weights.len()-1 { 
        println!("testing: {}", i);
        for group_weights in weights.iter().combinations(i) {
            if group_weights.iter().map(|&x| x).sum::<i64>() == expected_weight {
                should_break = true;
                
                let quantum_entanglement: i64 = group_weights.iter().map(|&x| x).product();
                lowest_quantum_entanglement = cmp::min(lowest_quantum_entanglement, quantum_entanglement);
            }
        }

        if should_break {
            break;
        }
    }

    println!("lowest quantum entanglement: {}", lowest_quantum_entanglement);
}