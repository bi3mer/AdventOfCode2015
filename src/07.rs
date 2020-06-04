use std::collections::HashMap;
use std::fs;
use std::env;

fn create_wires(input: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let lines = input.split("\n");

    for line in lines {
        let split_line = line.split("->").collect::<Vec<&str>>();
        let target = split_line[1].trim();

        if map.contains_key(target) {
            *map.get_mut(&target.to_string()).unwrap() = split_line[0].trim().to_string();
        } else {
           map.insert(target.to_string(), split_line[0].trim().to_string());
        }
    }

    map
}

fn solve(gate_id: String, wires: &HashMap<String, String>, cache: &mut HashMap<String, u16>) -> u16 {
    let return_val;

    if cache.contains_key(&gate_id) {
        return_val = *cache.get(&gate_id.to_string()).unwrap();
    } else {
        if !wires.contains_key(&gate_id) {
            panic!("Error: gate id '{}' was not found.", gate_id);
        }

        let operators = wires.get(&gate_id).unwrap().split(' ').collect::<Vec<&str>>();

        if operators.len() == 1 {
            // We have received a SET command
            match operators[0].parse::<u16>() {
                Ok(value) => {
                    return_val = value;
                },
                Err(_err) => {
                    return_val = solve(operators[0].to_string(), wires, cache);
                }
            } 
        } else if operators.len() == 2 {
            // We have received a NOT command and to run this we first need
            // to get the value for the target
            return_val = !solve(operators[1].to_string(), wires, cache);
        } else {
            let right;
            let left;


            match operators[0].parse::<u16>() {
                Ok(value) => {
                    left = value;
                },
                Err(_err) => {
                    left = solve(operators[0].to_string(), wires, cache);
                }
            }

            match operators[2].parse::<u16>() {
                Ok(value) => {
                    right = value;
                },
                Err(_err) => {
                    right = solve(operators[2].to_string(), wires, cache);
                }
            }

            match operators[1] {
                "OR" => return_val = left | right,
                "AND" => return_val = left & right,
                "RSHIFT" => return_val = left >> right,
                "LSHIFT" => return_val= left << right,
                _ => panic!("{} is not supported", operators[1])
            }
        }
    }

    cache.insert(gate_id.to_string(), return_val);
    return_val
}

fn main() {
    let file = format!("{}/input/input07.txt", env::current_dir().unwrap().to_str().unwrap());
    println!("{}", file);
    let content = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let mut wires = create_wires(&content);
    let mut cache = HashMap::new();

    let result =  solve("a".to_string(), &wires, &mut cache);
    println!("PART 1: {}", result);

    *wires.get_mut("b").unwrap() = result.to_string();

    cache.clear();
    println!("PART 2: {}", solve("a".to_string(), &wires, &mut cache));
}