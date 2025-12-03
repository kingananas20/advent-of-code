use std::fs;

const BATTERIES: usize = 12;

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").unwrap();
    let mut joltage = 0;

    for line in input.lines() {
        let digits: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let mut start = 0;

        for i in 0..BATTERIES {
            let end = digits.len() - (BATTERIES - i) + 1;

            let (index, number) = digits[start..end].iter().copied().first_max().unwrap();

            start += index + 1;

            joltage += (number as u64) * 10u64.pow((BATTERIES - i - 1) as u32);
        }
    }

    println!("Joltage: {joltage}");
}

trait IteratorExt: Iterator<Item = u8> + Sized {
    fn first_max(self) -> Option<(usize, u8)> {
        let mut max_val: Option<u8> = None;
        let mut max_idx: usize = 0;
        let mut idx: usize = 0;

        for val in self {
            if max_val.map_or(true, |m| val > m) {
                max_val = Some(val);
                max_idx = idx;
            }
            idx += 1;
        }

        max_val.map(|v| (max_idx, v))
    }
}

impl<I: Iterator<Item = u8>> IteratorExt for I {}
