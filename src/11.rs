fn is_valid_password(password: String) -> bool {
    let mut has_confusing_characters = false;
    let mut has_three_in_a_row = false;
    let mut has_two_doubles = false;
    let mut first_double_byte = 0;
    
    let bytes = password.into_bytes();
    let mut previous_previous_byte = bytes[0];
    let mut previous_byte = bytes[1];

    for byte_index in 2..bytes.len() {
        let current_byte = bytes[byte_index];

        if current_byte - 1 == previous_byte && previous_byte - 1 == previous_previous_byte {
            has_three_in_a_row = true;
        }
        
        if previous_byte == previous_previous_byte {
            if first_double_byte == 0 {
                first_double_byte = previous_byte;
            } else if first_double_byte != previous_byte {
                has_two_doubles = true;
            } 
        }

        if current_byte == 'i' as u8 || current_byte == 'o' as u8 || current_byte == 'l' as u8 {
            has_confusing_characters = true;
            break;
        }

        previous_previous_byte = previous_byte;
        previous_byte = current_byte;
    }

    // handle edge case where the last character could be a double character.
    if previous_byte == previous_previous_byte && 
       first_double_byte != 0 && 
       previous_byte != first_double_byte {
        
        has_two_doubles = true;
    }

    has_two_doubles && has_three_in_a_row && !has_confusing_characters
}

fn generate_next_password(password: String) -> String {
    let byte_a = 'a' as u8;
    let byte_z = 'z' as u8;
    
    let mut bytes = password.into_bytes();
    let mut index = bytes.len();

    loop {
        if bytes[index - 1] == byte_z {
            bytes[index - 1] = byte_a;
            index -= 1;
            
            if index == 0 {
                break;
            }
        } else {
            bytes[index - 1] += 1;
            break;
        }
    }

    String::from_utf8(bytes).unwrap()
}

fn generate_next_valid_password(password: String) -> String {
    let mut new_password = password;
    loop {
        new_password = generate_next_password(new_password);
        if is_valid_password(new_password.clone()) {
            break;
        }
    }

    new_password
}

fn main() {
    println!("is_valid_password tests:");
    println!("passed: {}", is_valid_password(String::from("hijklmmn")) == false);
    println!("passed: {}", is_valid_password(String::from("abbceffg")) == false);
    println!("passed: {}", is_valid_password(String::from("abbcegjk")) == false);
    println!("passed: {}", is_valid_password(String::from("abcdffaa")) == true);
    println!("passed: {}", is_valid_password(String::from("ghjaabcc")) == true);
    println!("passed: {}", is_valid_password(String::from("vzbxxxyz")) == false);
    println!("passed: {}", is_valid_password(String::from("abcdeggg")) == false);

    println!("\ngenerate_next_password tests:");
    println!("passed: {}", generate_next_password(String::from("xx")) == String::from("xy"));
    println!("passed: {}", generate_next_password(String::from("xy")) == String::from("xz"));
    println!("passed: {}", generate_next_password(String::from("xz")) == String::from("ya"));
    println!("passed: {}", generate_next_password(String::from("ya")) == String::from("yb"));
    println!("passed: {}", generate_next_password(String::from("azz")) == String::from("baa"));

    println!("\ngenerate_next_valid_password tests:");
    println!("{}", generate_next_valid_password(String::from("abcdefgh")) == String::from("abcdffaa"));
    println!("{}", generate_next_valid_password(String::from("ghijklmn")) == String::from("ghjaabcc"));

    let first_password = generate_next_valid_password(String::from("vzbxkghb"));
    let second_password = generate_next_valid_password(first_password.clone());

    println!("\nPart 1 Answer: {}", first_password);
    println!("Part 2 Answer: {}", second_password);
}