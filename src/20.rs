fn part1(house_index: u32) -> u32 {
    let mut presents_received: u32 = 0;
    let mut i: u32 = 1;

    while i <= (house_index as f32).sqrt().floor() as u32 {
        if house_index % i == 0 {
            presents_received += i;
            if house_index / i != i {
                presents_received += house_index / i;
            }
        }

        i += 1;
    }

    presents_received
}

fn part2(house_index: u32) -> u32 {
    let mut presents_received: u32 = 0;
    let mut elf_index: u32 = 1;

    while elf_index <= (house_index as f32).sqrt().floor() as u32 {
        if house_index % elf_index == 0 {
            if house_index / elf_index <= 50 {
                presents_received += elf_index;
            }

            let large_elf_index = house_index / elf_index;
            if large_elf_index != elf_index && house_index / large_elf_index <= 50 {
                presents_received += large_elf_index;
            }
        }

        elf_index += 1;
    }

    presents_received
}

fn main() {
    assert_eq!(part1(1), 1);
    assert_eq!(part1(2), 3);
    assert_eq!(part1(3), 4);
    assert_eq!(part1(4), 7);
    assert_eq!(part1(5), 6);
    assert_eq!(part1(6), 12);
    assert_eq!(part1(7), 8);
    assert_eq!(part1(8), 15);
    assert_eq!(part1(9), 13);

    let puzzle_input: u32 = 33100000;
    let mut house_number: u32 = 1;

    let part_1_puzzle_input = puzzle_input / 10;

    loop {
        if part1(house_number) >= part_1_puzzle_input {
            break;
        }

        house_number += 1;
    }

    println!("Part 1: {}", house_number);
        
    let part_2_puzzle_input = puzzle_input / 11;
    loop {
        if part2(house_number) >= part_2_puzzle_input {
            break;
        }

        house_number += 1;
    }

    println!("part 2: {}", house_number);
}