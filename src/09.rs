use std::collections::HashMap;
use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let file = format!("{}/input/input09.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file)
        .expect("Something went wrong reading the file.");
    
    let mut src_dst_distance: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    let mut destinations: Vec<&str> = Vec::new();
    
    let lines = content.split("\n");
    for line in lines {
        let locations_and_distance = line.split(" = ").collect::<Vec<&str>>();
        let locations = locations_and_distance[0].split(" to ").collect::<Vec<&str>>();

        let distance = locations_and_distance[1].parse::<u32>().unwrap();
        let location1 = locations[0];
        let location2 = locations[1];
        
        if src_dst_distance.contains_key(location1) {
            src_dst_distance.get_mut(location1).unwrap().insert(location2, distance);
        } else {
            let mut insert_hash = HashMap::new();
            insert_hash.insert(location2, distance);
            src_dst_distance.insert(location1, insert_hash);
        } 
        
        if src_dst_distance.contains_key(location2) {
            src_dst_distance.get_mut(location2).unwrap().insert(location1, distance);
        } else {
            let mut insert_hash = HashMap::new();
            insert_hash.insert(location1, distance);
            src_dst_distance.insert(location2, insert_hash);
        }

        if destinations.contains(&location1) == false {
            destinations.push(location1);
        }
        if destinations.contains(&location2) == false {
            destinations.push(location2);
        }
    }

    let mut lowest_distance = 1000000000;
    // let mut best_path: Vec<&&str>;
    for path in destinations.iter().permutations(destinations.len()) {
        let mut origin: &str = "";
        let mut path_distance = 0;

        for dst in path {
            if origin == "" {
                origin = dst;
            } else {
                path_distance += src_dst_distance.get(origin).unwrap().get(dst).unwrap();
                origin = dst;
            }
        }

        if path_distance < lowest_distance {
            lowest_distance = path_distance;
        }
    }

    // for dst in best_path {
    //     println!("{}", dst);
    // }
    println!("Distance: {}", lowest_distance)
}