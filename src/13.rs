use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let file = format!("{}/input/input13.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let mut people_to_happiness: HashMap<&str, HashMap<&str, i16>> = HashMap::new();
    let mut people = HashSet::new();
    
    for line in content.split('\n') {
        let split_line: Vec<&str> = line.split(" ").collect();
        let source = split_line[0];
        let modifier = split_line[2];
        let mut score: i16 = split_line[3].parse().unwrap();
        let target = split_line[10].split(".").nth(0).unwrap();
        
        if modifier == "lose" {
            score *= -1;
        }
        
        people.insert(source);
        people.insert(target);
        
        if people_to_happiness.contains_key(source) {
            people_to_happiness.get_mut(source).unwrap().insert(target, score);
        } else {
            let mut insert_hash = HashMap::new();
            insert_hash.insert(target, score);
            people_to_happiness.insert(source, insert_hash);
        }
    }

    // ----------- Comment out for part 1 results -----------
    let mut insert_hash: HashMap<&str, i16> = HashMap::new();
    for person in people.iter() {
        insert_hash.insert(person, 0);
        people_to_happiness.get_mut(person).unwrap().insert("me", 0);
    }

    people.insert("me");
    people_to_happiness.insert("me", insert_hash);
    // ------------------------------------------------------

    let last_index = people.len() - 1;
    let mut best_happiness = 0;
    for seating in people.iter().permutations(people.len()) {
        let mut happiness = 0;

        for index in 0..(last_index + 1) {
            let left_index = if index == 0 {last_index} else {index - 1};
            happiness += people_to_happiness.get(seating[index]).unwrap().get(seating[left_index]).unwrap();

            let right_index = if index + 1 <= last_index {index + 1} else {0};
            happiness += people_to_happiness.get(seating[index]).unwrap().get(seating[right_index]).unwrap();
        }

        if happiness > best_happiness {
            best_happiness = happiness;
        }
    }

    println!("Answer: {}", best_happiness);
}
