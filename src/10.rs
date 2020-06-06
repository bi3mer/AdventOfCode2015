fn part1(input: String) -> String {
    let mut result = String::from("");
    let mut count = 1;
    
    let mut char_iterator = input.chars();
    let mut previous_char = char_iterator.next().unwrap();

    for character in char_iterator {
        if character != previous_char {
            result.push_str(&count.to_string());
            result.push_str(&previous_char.to_string());

            previous_char = character;
            count = 1;
        } else {
            count += 1;
        }
    }

    result.push_str(&count.to_string());
    result.push_str(&previous_char.to_string());

    result
}

fn main() {
    println!("{}", part1(String::from("1")) == String::from("11"));
    println!("{}", part1(String::from("11")) == String::from("21"));
    println!("{}", part1(String::from("21")) == String::from("1211"));
    println!("{}", part1(String::from("1211")) == String::from("111221"));

    let mut input = String::from("1113122113");
    for _ in 0..50 {
        input = part1(input);
    }

    println!("{}", input.len());
}