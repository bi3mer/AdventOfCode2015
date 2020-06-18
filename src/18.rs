use std::env;
use std::fs;

fn get_increment(light_matrix: &Vec<Vec<bool>>, row: usize, col: usize) -> u8 {
    if *light_matrix.get(row).unwrap().get(col).unwrap() == true {
        return 1;
    }

    return 0;
}

fn num_alive_neighbors(light_matrix: &Vec<Vec<bool>>, row: usize, col: usize, height: usize, width: usize) -> u8 {
    let mut num_alive: u8 = 0;

    // -1 -1
    if row != 0 && col != 0 {
        num_alive += get_increment(light_matrix, row - 1, col - 1);
    }

    // -1 0
    if row != 0 {
        num_alive += get_increment(light_matrix, row - 1, col);
    }

    // -1 1
    if row != 0 && col != width - 1 {
        num_alive += get_increment(light_matrix, row - 1, col + 1);
    }

    // 0 -1
    if col != 0 {
        num_alive += get_increment(light_matrix, row, col - 1);
    }

    // 0 1
    if col != width - 1 {
        num_alive += get_increment(light_matrix, row, col + 1);
    }
    
    // 1 -1
    if row != height - 1 && col != 0 {
        num_alive += get_increment(light_matrix, row + 1, col - 1);
    }

    // 1 0
    if row != height - 1 {
        num_alive += get_increment(light_matrix, row + 1, col);
    }

    // 1 1
    if row != height - 1 && col != width - 1 {
        num_alive += get_increment(light_matrix, row + 1, col + 1);
    }

    num_alive
}

fn main() {
    // set is_part_two to false to see output for part 1.
    let is_part_two = true; 
    let print_steps = false;

    // let file = format!("{}/input/input18_test.txt", env::current_dir().unwrap().to_str().unwrap());
    // let steps = 5;

    let file = format!("{}/input/input18.txt", env::current_dir().unwrap().to_str().unwrap());
    let steps = 100;

    let mut light_matrix: Vec<Vec<bool>> = Vec::new();

    let content = fs::read_to_string(file).expect("Something went wrong reading the file");
    for line in content.split("\n") {
        let mut light_row: Vec<bool> = Vec::new();
        for character in line.chars() {
            match character {
                '#' => light_row.push(true),
                _ => light_row.push(false)
            }
        }

        light_matrix.push(light_row);
    }

    let height = light_matrix.len();
    let width = light_matrix.get(0).unwrap().len();

    if is_part_two {
        *light_matrix.get_mut(0).unwrap().get_mut(0).unwrap() = true;
        *light_matrix.get_mut(height - 1).unwrap().get_mut(0).unwrap() = true;
        *light_matrix.get_mut(0).unwrap().get_mut(width - 1).unwrap() = true;
        *light_matrix.get_mut(height - 1).unwrap().get_mut(width - 1).unwrap() = true;
    }

    for step in 1..steps+1 {
        let mut new_light_matrix: Vec<Vec<bool>> = Vec::new();

        for row_index in 0..light_matrix.len() {
            let light_row = light_matrix.get(row_index).unwrap();
            let mut new_light_row: Vec<bool> = Vec::new();

            for col_index in 0..light_matrix.len() {
                let alive_neighbors: u8 = num_alive_neighbors(
                    &light_matrix, 
                    row_index, 
                    col_index, 
                    height, 
                    width);

                if *light_row.get(col_index).unwrap() == true {
                    if alive_neighbors == 2 || alive_neighbors == 3 {
                        new_light_row.push(true);
                    } else {
                        new_light_row.push(false);
                    }
                } else {
                    if alive_neighbors == 3 {
                        new_light_row.push(true);
                    } else {
                        new_light_row.push(false);
                    }
                }
            }

            new_light_matrix.push(new_light_row);
        }

        light_matrix = new_light_matrix;

        if is_part_two {
            *light_matrix.get_mut(0).unwrap().get_mut(0).unwrap() = true;
            *light_matrix.get_mut(height - 1).unwrap().get_mut(0).unwrap() = true;
            *light_matrix.get_mut(0).unwrap().get_mut(width - 1).unwrap() = true;
            *light_matrix.get_mut(height - 1).unwrap().get_mut(width - 1).unwrap() = true;
        }

        if print_steps {
            println!("Step {}:", step);
            for row in light_matrix.iter() {
                let mut light_row = String::from("");
                for is_active in row.iter() {
                    match is_active {
                        true => light_row.push_str("#"),
                        _ => light_row.push_str(".")
                    }
                }

                println!("{}", light_row);
            }

            println!("");
        }
    }

    let mut alive_lights = 0;
    for light_row in light_matrix.iter() {
        for is_on in light_row.iter() {
            if *is_on == true {
                alive_lights += 1;
            }
        }
    }

    println!("Alive Lights: {}", alive_lights);
}