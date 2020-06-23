fn house_received(house_index: u32) -> u32 {
    let mut presents_received: u32 = 0;
    let mut i: u32 = 1;

    while i <= (house_index as f32).sqrt().floor() as u32 {
        if house_index % i == 0 {
            if house_index / i == i {
                presents_received += i;
            } else {
                presents_received += i;
                presents_received += house_index / i;
            }
        }

        i += 1;
    }

    presents_received * 10
}

fn main() {
    assert_eq!(house_received(1), 10);
    assert_eq!(house_received(2), 30);
    assert_eq!(house_received(3), 40);
    assert_eq!(house_received(4), 70);
    assert_eq!(house_received(5), 60);
    assert_eq!(house_received(6), 120);
    assert_eq!(house_received(7), 80);
    assert_eq!(house_received(8), 150);
    assert_eq!(house_received(9), 130);

    let puzzle_input: u32 = 33100000;
    let mut house_number: u32 = 1;

    loop {
        if house_received(house_number) >= puzzle_input {
            break;
        }

        house_number += 1;
    }

    println!("Part 1: {}", house_number);
}