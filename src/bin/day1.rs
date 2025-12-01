use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();

    let mut dial: u8 = 50;
    let mut password = 0;

    for line in input.lines() {
        let b = line.get(1..).unwrap().parse().unwrap();
        if line.starts_with("L") {
            let (a, at_zero) = sub(dial, b);
            dial = a;
            password += at_zero;
        } else if line.starts_with("R") {
            let (a, at_zero) = add(dial, b);
            dial = a;
            password += at_zero;
        } else {
            continue;
        }
    }

    println!("The password is: {password}");
}

const UPPER_BOUND: u8 = 99;

fn add(mut dial: u8, b: u32) -> (u8, u32) {
    let mut at_zero = 0;

    for _ in 0..b {
        dial += 1;
        if dial == UPPER_BOUND + 1 {
            dial = 0;
        }
        if dial == 0 {
            at_zero += 1;
        }
    }

    (dial, at_zero)
}

fn sub(mut dial: u8, b: u32) -> (u8, u32) {
    let mut at_zero = 0;

    for _ in 0..b {
        dial = dial.wrapping_sub(1);
        if dial == 0 {
            at_zero += 1;
        }
        if dial == u8::MAX {
            dial = UPPER_BOUND;
        }
    }

    (dial, at_zero)
}
