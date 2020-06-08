use serde_json::{Map, Value};
use std::env;
use std::fs;

///////////// Part 1
fn part1_dictionary_sum(map: &Map<String, Value>) -> i64 {
    let mut sum = 0;
    for (_key, value) in map.iter() {
        if value.is_i64() {
            sum += value.as_i64().unwrap();
        } else if value.is_string() == false {
            sum += part1_json_sum(value.clone());
        }
    }

    sum
}

fn part1_array_sum(list: &Vec<Value>) -> i64 {
    let mut sum = 0;
    for value in list {
        if value.is_i64() {
            sum += value.as_i64().unwrap();
        } else if value.is_string() == false {
            sum += part1_json_sum(value.clone());
        }
    }

    sum
}

fn part1_json_sum(json: Value) -> i64 {
    if json.is_array() {
        part1_array_sum(json.as_array().unwrap())
    } else {
        part1_dictionary_sum(json.as_object().unwrap()) 
    }
}

fn part1_get_sum(json_str: String) -> i64 {
    part1_json_sum(serde_json::from_str(&json_str).unwrap())
}

///////////// Part 2
fn part2_dictionary_sum(map: &Map<String, Value>) -> i64 {
    let mut sum = 0;
    for (_key, value) in map.iter() {
        if value.is_i64() {
            sum += value.as_i64().unwrap();
        } else if value.is_string() == false {
            sum += part2_json_sum(value.clone());
        } else {
            let string_value = value.as_str().unwrap();
            if string_value == "red" {
                sum = 0;
                break;
            }
        }
    }

    sum
}

fn part2_array_sum(list: &Vec<Value>) -> i64 {
    let mut sum = 0;
    for value in list {
        if value.is_i64() {
            sum += value.as_i64().unwrap();
        } else if value.is_string() == false {
            sum += part2_json_sum(value.clone());
        }
    }

    sum
}

fn part2_json_sum(json: Value) -> i64 {
    if json.is_array() {
        part2_array_sum(json.as_array().unwrap())
    } else {
        part2_dictionary_sum(json.as_object().unwrap()) 
    }
}

fn part2_get_sum(json_str: String) -> i64 {
    part2_json_sum(serde_json::from_str(&json_str).unwrap())
}

fn main() {
    println!("Part 1 Tests:");
    println!("{}", part1_get_sum(String::from(r#"[1,2,3]"#)) == 6);
    println!("{}", part1_get_sum(String::from(r#"{"a":2,"b":4}"#)) == 6);
    println!("{}", part1_get_sum(String::from(r#"[[[3]]]"#)) == 3);
    println!("{}", part1_get_sum(String::from(r#"{"a":{"b":4},"c":-1}"#)) == 3);
    println!("{}", part1_get_sum(String::from(r#"{"a":[-1,1]}"#)) == 0);
    println!("{}", part1_get_sum(String::from(r#"[-1,{"a":1}]"#)) == 0);
    println!("{}", part1_get_sum(String::from(r#"[]"#)) == 0);
    println!("{}", part1_get_sum(String::from(r#"{}"#)) == 0);

    println!("\nPart 2 Tests:");
    println!("{}", part2_get_sum(String::from(r#"[1,2,3]"#)) == 6);
    println!("{}", part2_get_sum(String::from(r#"[[[3]]]"#)) == 3);
    println!("{}", part2_get_sum(String::from(r#"{"a":[-1,1]}"#)) == 0);
    println!("{}", part2_get_sum(String::from(r#"[-1,{"a":1}]"#)) == 0);
    println!("{}", part2_get_sum(String::from(r#"[]"#)) == 0);
    println!("{}", part2_get_sum(String::from(r#"[1,"red",5]"#)) == 6);
    println!("{}", part2_get_sum(String::from(r#"[1,{"c":"red","b":2},3]"#)) == 4);
    println!("{}", part2_get_sum(String::from(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)) == 0);

    let file = format!("{}/input/input12.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    
    println!("");
    println!("Part 1 Answer: {}", part1_get_sum(content.clone()));
    println!("Part 2 Answer: {}", part2_get_sum(content.clone()));
}