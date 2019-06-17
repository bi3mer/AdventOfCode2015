use std::collections::HashSet;
use std::fs;

#[derive(Hash)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new(x: i32, y: i32) -> Position{
        Position {x: x, y: y}
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool{
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

fn part_1_solution(input: &String) -> usize{
    let mut positions = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    positions.insert(Position::new(0, 0));

    for c in input.chars() {
        match c {
            '<' => { x -= 1;}
            '>' => { x += 1;}
            '^' => { y += 1;}
            'v' => { y -= 1;}
            _ => {}
        }

        positions.insert(Position::new(x, y));
    }

    positions.len()
}

fn part_2_solution(input: &String) -> usize {
    let mut positions = HashSet::new();
    positions.insert(Position::new(0, 0));

    let mut regular_santa_x = 0;
    let mut regular_santa_y = 0;
    
    let mut robot_santa_x = 0;
    let mut robot_santa_y = 0;

    let mut santas_turn = true;

    for c in input.chars() {
        if santas_turn {
            match c {
                '<' => { regular_santa_x -= 1; }
                '>' => { regular_santa_x += 1; }
                '^' => { regular_santa_y += 1; }
                'v' => { regular_santa_y -= 1; }
                _ => {}
            }

            positions.insert(Position::new(regular_santa_x, regular_santa_y));
        } else {
            match c {
                '<' => { robot_santa_x -= 1; }
                '>' => { robot_santa_x += 1; }
                '^' => { robot_santa_y += 1; }
                'v' => { robot_santa_y -= 1; }
                _ => {}
            }

            positions.insert(Position::new(robot_santa_x, robot_santa_y));
        }

        santas_turn = !santas_turn;
    }

    positions.len()
}

fn main() {
    println!("test case 0 passed: {}", part_1_solution(&">".to_string()) == 2);
    println!("test case 1 passed: {}", part_1_solution(&"^>v<".to_string()) == 4);
    println!("test case 2 passed: {}", part_1_solution(&"^v^v^v^v^v".to_string()) == 2);

    let file_name = "input.txt";
    let content = fs::read_to_string(file_name)
        .expect("Unable to read file for some reason");

    println!("Part 1 Answer: {}", part_1_solution(&content));

    println!("\n");
    println!("test case 3 passed: {}", part_2_solution(&"^v".to_string()) == 3);
    println!("test case 4 passed: {}", part_2_solution(&"^>v<".to_string()) == 3);
    println!("test case 5 passed: {}", part_2_solution(&"^v^v^v^v^v".to_string()) == 11);

    println!("Part 2 Answer: {}", part_2_solution(&content));
}