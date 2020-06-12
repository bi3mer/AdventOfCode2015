use std::env;
use std::fs;

fn main() {
    let children = 3;
    let cats = 7;
    let samoyeds = 2;
    let pomeranians = 3;
    let akitas = 0;
    let vizslas = 0;
    let goldfish = 5;
    let trees = 3;
    let cars = 2;
    let perfumes = 1;

    let file = format!("{}/input/input16.txt", env::current_dir().unwrap().to_str().unwrap());
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut part_1_best_score = 0;
    let mut part_1_best_aunt = "0";

    let mut part_2_best_score = 0;
    let mut part_2_best_aunt = "0";

    for line in content.split("\n") {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let mut similarity_1 = 0;
        let mut similarity_2 = 0;
        
        for i in (2..split_line.len()).step_by(2) {
            let num = split_line[i + 1].replace(",", "").parse::<i16>().unwrap();

            match split_line[i] {
                "children:" => {
                    if num == children {
                        similarity_1 += 1;
                        similarity_2 += 1;
                    }
                },
                "cats:" => {
                    if num == cats {
                        similarity_1 += 1;
                    }

                    if num >= cats {
                        similarity_2 += 1;
                    }
                },
                "samoyeds:" => {
                    if num == samoyeds {
                        similarity_1 += 1;
                        similarity_2 += 1;
                    }
                },
                "pomeranians:" => {
                    if num == pomeranians {
                        similarity_1 += 1;
                    }

                    if num < pomeranians {
                        similarity_2 += 1;
                    }
                },
                "akitas:" => {
                    if num == akitas {
                        similarity_1 += 1;
                        similarity_2 += 1;
                    }
                },
                "vizslas:" => {
                    if num == vizslas {
                        similarity_1 += 1;
                        similarity_2 += 1;
                    }
                },
                "goldfish:" => {
                    if num == goldfish {
                        similarity_1 += 1;
                    }

                    if num < goldfish {
                        similarity_2 += 1;
                    }
                },
                "trees:" => {
                    if num == trees {
                        similarity_1 += 1;
                    }

                    if num >= trees {
                        similarity_2 += 1;
                    }
                },
                "cars:" => {
                    if num == cars {
                        similarity_1 += 1;
                        similarity_2 += 1;
                    }
                },
                "perfumes:" => {
                    if num == perfumes {
                        similarity_1 += 1;
                        similarity_2 += 1;
                    }
                },
                _ => println!("{} not handled.", split_line[i])
            }
        }

        if similarity_1 > part_1_best_score {
            part_1_best_score = similarity_1;
            part_1_best_aunt = split_line[1];
        }

        if similarity_2 > part_2_best_score {
            part_2_best_score = similarity_2;
            part_2_best_aunt = split_line[1];
        }
    }

    println!("Part 1: aunt {} has similarity {}", part_1_best_aunt, part_1_best_score);
    println!("Part 2: aunt {} has similarity {}", part_2_best_aunt, part_2_best_score);
}