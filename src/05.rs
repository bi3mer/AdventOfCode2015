use std::io::{BufRead, BufReader};
use std::fs::File;

fn is_naughty_01(input: &String) -> bool {
    let vowel_count = input.matches("a").count() + 
                      input.matches("e").count() + 
                      input.matches("i").count() + 
                      input.matches("o").count() +
                      input.matches("u").count();

    if vowel_count < 3 {
        return true
    }

    let bad_combinations = ["ab", "cd", "pq", "xy"];
    for bad in bad_combinations.iter() {
        if input.matches(bad).count() > 0 {
            return true
        }
    }

    let mut past_char = 'a';
    let mut has_double_character = true;

    for c in input.chars() {
        if has_double_character {
            has_double_character = false;
        } else if past_char == c {
            has_double_character = true;
            break
        }

        past_char = c;
    }

    !has_double_character
}

fn is_naughty_02(input: &String) -> bool {
    let chars = input.chars();

    // find two with one in between (aba not aab)
    let mut valid_in_between_found = false;
    let mut valid_pair_set_found = false;

    let mut past_1_found = false;
    let mut past_2_found = false;

    let mut past_char_1 = 'a';
    let mut past_char_2 = 'a';

    for c in chars {
        if !past_1_found {
            past_1_found = true;
            past_char_1 = c;
        } else if !past_2_found {
            past_2_found = true;
            past_char_2 = c;
        } else {
            if !valid_in_between_found && c == past_char_1 {
                valid_in_between_found = true;
            }

            let pair = format!("{}{}", past_char_1, past_char_2);
            if !valid_pair_set_found && input.matches(&pair).count() > 1 {
                valid_pair_set_found = true;
            }

            if valid_in_between_found && valid_pair_set_found {
                break
            }

            past_char_1 = past_char_2;
            past_char_2 = c;
        }
    }

    !valid_in_between_found || !valid_pair_set_found
}

fn main() {
    println!("Test Cases 01");
    println!("false == {}", is_naughty_01(&"ugknbfddgicrmopn".to_string()));
    println!("false == {}", is_naughty_01(&"aaa".to_string()));
    println!("true  == {}", is_naughty_01(&"jchzalrnumimnmhp".to_string()));
    println!("true  == {}", is_naughty_01(&"haegwjzuvuyypxyu".to_string()));
    println!("true  == {}", is_naughty_01(&"dvszwmarrgswjxmb".to_string()));

    println!("\nTest Cases 02");
    println!("false == {}", is_naughty_02(&"qjhvhtzxzqqjkmpb".to_string()));
    println!("false == {}", is_naughty_02(&"xxyxx".to_string()));
    println!("true  == {}", is_naughty_02(&"uurcxstgmygtbstg".to_string()));
    println!("true  == {}", is_naughty_02(&"ieodomkazucvgmuy".to_string()));

    let mut count01 = 0;
    let mut count02 = 0;

    let file_name = "input/input05.txt";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if !is_naughty_01(&line) {
            count01 += 1;
        }

        if !is_naughty_02(&line) {
            count02 += 1;
        }
    }

    println!("\nOccurrences01: {}", count01);
    println!("Occurrences02: {}", count02); // correct answer is 55
}