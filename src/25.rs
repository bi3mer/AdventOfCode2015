fn run(target_row: u128, target_col: u128) -> u128{
    let mut current_val: u128 = 20151125;
    let multiplier: u128 = 252533;
    let divider: u128    = 33554393;
    
    let target_code = ((target_row + target_col - 1) * (target_row + target_col) / 2) - target_row + 1;
    for _ in 1..target_code {
        current_val = (current_val * multiplier) % divider;
    }

    current_val
}

fn main() {
    assert_eq!(20151125, run(1,1));
    assert_eq!(31916031, run(2,1));
    assert_eq!(16080970, run(3,1));
    assert_eq!(24592653, run(4,1));
    assert_eq!(77061,    run(5,1));
    assert_eq!(33071741, run(6,1));

    assert_eq!(18749137, run(1,2));
    assert_eq!(21629792, run(2,2));
    assert_eq!(8057251,  run(3,2));
    assert_eq!(32451966, run(4,2));
    assert_eq!(17552253, run(5,2));
    assert_eq!(6796745,  run(6,2));

    assert_eq!(17289845, run(1,3));
    assert_eq!(16929656, run(2,3));
    assert_eq!(1601130,  run(3,3));
    assert_eq!(21345942, run(4,3));
    assert_eq!(28094349, run(5,3));
    assert_eq!(25397450, run(6,3));

    assert_eq!(30943339, run(1,4));
    assert_eq!(7726640,  run(2,4));
    assert_eq!(7981243,  run(3,4));
    assert_eq!(9380097,  run(4,4));
    assert_eq!(6899651,  run(5,4));
    assert_eq!(24659492, run(6,4));

    assert_eq!(10071777, run(1,5));
    assert_eq!(15514188, run(2,5));
    assert_eq!(11661866, run(3,5));
    assert_eq!(10600672, run(4,5));
    assert_eq!(9250759,  run(5,5));
    assert_eq!(1534922,  run(6,5));

    assert_eq!(33511524, run(1,6));
    assert_eq!(4041754,  run(2,6));
    assert_eq!(16474243, run(3,6));
    assert_eq!(31527494, run(4,6));
    assert_eq!(31663883, run(5,6));
    assert_eq!(27995004, run(6,6));
    

    println!("result: {}", run(2978, 3083))
}