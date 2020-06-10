use std::collections::HashMap;
use std::env;
use std::fs;

struct Info {
    km_per_sec: u16,
    run_period: u16,
    rest_period: u16
}

struct State {
    running: bool,
    time_left_in_state: u16,
    position: u16
}

fn main() {
    let seconds = 2503;

    let file = format!("{}/input/input14.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file)
    .expect("Something went wrong reading the file");

    let mut name_to_score: HashMap<&str, u16> = HashMap::new();
    let mut name_to_info: HashMap<&str, Info> = HashMap::new();
    let mut name_to_state: HashMap<&str, State> = HashMap::new();

    for line in content.split("\n") {
        let split_line: Vec<&str> = line.split(" ").collect();
        let name = split_line[0];
        let speed: u16 = split_line[3].parse().unwrap();
        let run: u16 = split_line[6].parse().unwrap();
        let rest: u16 = split_line[13].parse().unwrap();

        name_to_score.insert(name, 0);

        name_to_info.insert(name, Info {
            km_per_sec:  speed,
            run_period:  run,
            rest_period: rest
        });

        name_to_state.insert(name, State {
            running: true,
            time_left_in_state: run,
            position: 0
        });
    }

    for _seconds in 0..seconds {
        for (name, state) in name_to_state.iter_mut() {
            let info = name_to_info.get(name).unwrap();

            if state.running {
                state.position += info.km_per_sec;
            }

            state.time_left_in_state -= 1;
            if state.time_left_in_state == 0 {
                if state.running {
                    state.time_left_in_state = info.rest_period;
                } else {
                    state.time_left_in_state = info.run_period;
                }

                state.running = !state.running;
            }
        }

        let mut largest_distance = 0;
        let mut best_reindeer = "";

        for (name, state) in name_to_state.iter() {
            if state.position > largest_distance {
                best_reindeer = name;
                largest_distance = state.position;
            }
        }

        let score = name_to_score.entry(best_reindeer).or_insert(0);
        *score += 1;
    }

    let mut largest_distance = 0;
    for (_name, state) in name_to_state.iter() {
        if state.position > largest_distance {
            largest_distance = state.position;
        }
    }

    println!("\nBest Distance: {}", largest_distance);

    let mut max_score = 0;
    for (_name, score) in name_to_score {
        if score > max_score {
            max_score = score;
        }
    }

    println!("Most points: {}", max_score);
}